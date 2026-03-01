#![cfg(feature = "integration")]

//! Integration tests against live MQ queue managers.
//!
//! These tests require running MQ containers started by `./scripts/dev/mq_start.sh`
//! and seeded by `./scripts/dev/mq_seed.sh`.
//!
//! Run with: `cargo test --features integration`

use std::collections::HashMap;
use std::env;

use mq_rest_admin::auth::Credentials;
use mq_rest_admin::ensure::EnsureAction;
use mq_rest_admin::error::MqRestError;
use mq_rest_admin::session::MqRestSession;
use serde_json::{Value, json};

// ---------------------------------------------------------------------------
// Seeded objects (created by mq_seed.sh)
// ---------------------------------------------------------------------------

const SEEDED_QUEUES: &[&str] = &[
    "DEV.DEAD.LETTER",
    "DEV.QLOCAL",
    "DEV.QREMOTE",
    "DEV.QALIAS",
    "DEV.QMODEL",
    "DEV.XMITQ",
];

const SEEDED_CHANNELS: &[&str] = &["DEV.SVRCONN", "DEV.SDR", "DEV.RCVR"];

const SEEDED_LISTENER: &str = "DEV.LSTR";
const SEEDED_TOPIC: &str = "DEV.TOPIC";
const SEEDED_NAMELIST: &str = "DEV.NAMELIST";
const SEEDED_PROCESS: &str = "DEV.PROC";

// ---------------------------------------------------------------------------
// Test objects (created/destroyed per test)
// ---------------------------------------------------------------------------

const TEST_QLOCAL: &str = "DEV.TEST.QLOCAL";
const TEST_QREMOTE: &str = "DEV.TEST.QREMOTE";
const TEST_QALIAS: &str = "DEV.TEST.QALIAS";
const TEST_QMODEL: &str = "DEV.TEST.QMODEL";
const TEST_CHANNEL: &str = "DEV.TEST.SVRCONN";
const TEST_LISTENER: &str = "DEV.TEST.LSTR";
const TEST_PROCESS: &str = "DEV.TEST.PROC";
const TEST_TOPIC: &str = "DEV.TEST.TOPIC";
const TEST_NAMELIST: &str = "DEV.TEST.NAMELIST";
const TEST_ENSURE_QLOCAL: &str = "DEV.ENSURE.QLOCAL";
const TEST_ENSURE_CHANNEL: &str = "DEV.ENSURE.CHL";

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Default admin identity for the MQ developer container (non-secret, local-only).
const MQ_DEV_ADMIN_IDENTITY: &str = "mqadmin";

struct IntegrationConfig {
    rest_base_url: String,
    admin_user: String,
    admin_password: String,
    qmgr_name: String,
    qm2_rest_base_url: String,
    qm2_qmgr_name: String,
}

fn load_config() -> IntegrationConfig {
    let default_identity = MQ_DEV_ADMIN_IDENTITY.to_owned();
    IntegrationConfig {
        rest_base_url: env::var("MQ_REST_BASE_URL")
            .unwrap_or_else(|_| "https://localhost:9443/ibmmq/rest/v2".into()),
        admin_user: env::var("MQ_ADMIN_USER").unwrap_or_else(|_| default_identity.clone()),
        admin_password: env::var("MQ_ADMIN_PASSWORD").unwrap_or_else(|_| default_identity),
        qmgr_name: env::var("MQ_QMGR_NAME").unwrap_or_else(|_| "QM1".into()),
        qm2_rest_base_url: env::var("MQ_REST_BASE_URL_QM2")
            .unwrap_or_else(|_| "https://localhost:9444/ibmmq/rest/v2".into()),
        qm2_qmgr_name: env::var("MQ_QMGR_NAME_QM2").unwrap_or_else(|_| "QM2".into()),
    }
}

// ---------------------------------------------------------------------------
// Session builders
// ---------------------------------------------------------------------------

fn build_session(config: &IntegrationConfig) -> MqRestSession {
    MqRestSession::builder(
        &config.rest_base_url,
        &config.qmgr_name,
        Credentials::Basic {
            username: config.admin_user.clone(),
            password: config.admin_password.clone(),
        },
    )
    .verify_tls(false)
    .build()
    .expect("failed to build session")
}

fn build_session_non_strict(config: &IntegrationConfig) -> MqRestSession {
    MqRestSession::builder(
        &config.rest_base_url,
        &config.qmgr_name,
        Credentials::Basic {
            username: config.admin_user.clone(),
            password: config.admin_password.clone(),
        },
    )
    .verify_tls(false)
    .mapping_strict(false)
    .build()
    .expect("failed to build session")
}

fn build_gateway_session(
    config: &IntegrationConfig,
    target_qmgr: &str,
    gateway_qmgr: &str,
    rest_base_url: &str,
) -> MqRestSession {
    MqRestSession::builder(
        rest_base_url,
        target_qmgr,
        Credentials::Basic {
            username: config.admin_user.clone(),
            password: config.admin_password.clone(),
        },
    )
    .gateway_qmgr(gateway_qmgr)
    .verify_tls(false)
    .build()
    .expect("failed to build gateway session")
}

// ---------------------------------------------------------------------------
// Assertion helpers
// ---------------------------------------------------------------------------

fn contains_string_value(map: &HashMap<String, Value>, expected: &str) -> bool {
    let normalized = expected.trim().to_uppercase();
    map.values().any(|value| {
        if let Some(s) = value.as_str() {
            s.trim().to_uppercase() == normalized
        } else {
            false
        }
    })
}

fn assert_results_contain(results: &[HashMap<String, Value>], expected: &str) {
    assert!(
        results
            .iter()
            .any(|result| contains_string_value(result, expected)),
        "expected to find '{expected}' in results"
    );
}

fn get_attribute_insensitive<'a>(map: &'a HashMap<String, Value>, key: &str) -> Option<&'a Value> {
    let upper = key.to_uppercase();
    map.iter()
        .find(|(k, _)| k.to_uppercase() == upper)
        .map(|(_, v)| v)
}

fn params(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_owned(), v.clone()))
        .collect()
}

fn verify_object_gone<F>(session: &mut MqRestSession, name: &str, display_fn: F)
where
    F: FnOnce(&mut MqRestSession, &str) -> Result<Vec<HashMap<String, Value>>, MqRestError>,
{
    match display_fn(session, name) {
        Err(_) => {} // object not found — expected
        Ok(results) => {
            assert!(
                !results.iter().any(|r| contains_string_value(r, name)),
                "object '{name}' should have been deleted but still appears in results"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Config validation
// ---------------------------------------------------------------------------

#[test]
fn config_defaults_are_valid() {
    let config = load_config();
    assert!(
        config.rest_base_url.starts_with("https://"),
        "REST base URL should start with https://"
    );
    assert!(
        !config.admin_user.is_empty(),
        "admin user should not be empty"
    );
    assert!(
        !config.admin_password.is_empty(),
        "admin password should not be empty"
    );
    assert!(
        !config.qmgr_name.is_empty(),
        "qmgr name should not be empty"
    );
}

// ---------------------------------------------------------------------------
// Singleton display tests
// ---------------------------------------------------------------------------

#[test]
fn display_qmgr_returns_object() {
    let config = load_config();
    let mut session = build_session(&config);

    let result = session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");

    assert!(result.is_some(), "display_qmgr should return an object");
    let qmgr = result.unwrap();
    assert!(
        contains_string_value(&qmgr, &config.qmgr_name),
        "qmgr object should contain the queue manager name"
    );
}

#[test]
fn display_qmstatus_returns_object_or_none() {
    let config = load_config();
    let mut session = build_session(&config);

    let result = session
        .display_qmstatus(None, None)
        .expect("display_qmstatus failed");

    // None is acceptable — some MQ versions don't return status
    if let Some(ref status) = result {
        assert!(
            !status.is_empty(),
            "if returned, qmstatus should not be empty"
        );
    }
}

#[test]
fn display_cmdserv_returns_object_or_none() {
    let config = load_config();
    let mut session = build_session(&config);

    let result = session
        .display_cmdserv(None, None)
        .expect("display_cmdserv failed");

    if let Some(ref cmdserv) = result {
        assert!(
            !cmdserv.is_empty(),
            "if returned, cmdserv should not be empty"
        );
    }
}

// ---------------------------------------------------------------------------
// Seeded object display tests — queues (macro)
// ---------------------------------------------------------------------------

macro_rules! test_display_seeded_queue {
    ($test_name:ident, $queue_index:expr) => {
        #[test]
        fn $test_name() {
            let config = load_config();
            let mut session = build_session(&config);
            let queue_name = SEEDED_QUEUES[$queue_index];

            let results = session
                .display_queue(Some(queue_name), None, None, None)
                .expect("display_queue failed");

            assert!(!results.is_empty(), "display_queue should return results");
            assert_results_contain(&results, queue_name);
        }
    };
}

test_display_seeded_queue!(display_seeded_queue_dead_letter, 0);
test_display_seeded_queue!(display_seeded_queue_qlocal, 1);
test_display_seeded_queue!(display_seeded_queue_qremote, 2);
test_display_seeded_queue!(display_seeded_queue_qalias, 3);
test_display_seeded_queue!(display_seeded_queue_qmodel, 4);
test_display_seeded_queue!(display_seeded_queue_xmitq, 5);

// ---------------------------------------------------------------------------
// Seeded object display tests — channels (macro)
// ---------------------------------------------------------------------------

macro_rules! test_display_seeded_channel {
    ($test_name:ident, $channel_index:expr) => {
        #[test]
        fn $test_name() {
            let config = load_config();
            let mut session = build_session(&config);
            let channel_name = SEEDED_CHANNELS[$channel_index];

            let results = session
                .display_channel(Some(channel_name), None, None, None)
                .expect("display_channel failed");

            assert!(!results.is_empty(), "display_channel should return results");
            assert_results_contain(&results, channel_name);
        }
    };
}

test_display_seeded_channel!(display_seeded_channel_svrconn, 0);
test_display_seeded_channel!(display_seeded_channel_sdr, 1);
test_display_seeded_channel!(display_seeded_channel_rcvr, 2);

// ---------------------------------------------------------------------------
// Seeded object display tests — individual
// ---------------------------------------------------------------------------

#[test]
fn display_qstatus_returns_object() {
    let config = load_config();
    let mut session = build_session(&config);

    let results = session
        .display_qstatus(Some("DEV.QLOCAL"), None, None, None)
        .expect("display_qstatus failed");

    assert!(!results.is_empty(), "display_qstatus should return results");
    assert_results_contain(&results, "DEV.QLOCAL");
}

#[test]
fn display_seeded_listener() {
    let config = load_config();
    let mut session = build_session(&config);

    let results = session
        .display_listener(Some(SEEDED_LISTENER), None, None, None)
        .expect("display_listener failed");

    assert!(
        !results.is_empty(),
        "display_listener should return results"
    );
    assert_results_contain(&results, SEEDED_LISTENER);
}

#[test]
fn display_seeded_topic() {
    let config = load_config();
    let mut session = build_session(&config);

    let results = session
        .display_topic(Some(SEEDED_TOPIC), None, None, None)
        .expect("display_topic failed");

    assert!(!results.is_empty(), "display_topic should return results");
    assert_results_contain(&results, SEEDED_TOPIC);
}

#[test]
fn display_seeded_namelist() {
    let config = load_config();
    let mut session = build_session(&config);

    let results = session
        .display_namelist(Some(SEEDED_NAMELIST), None, None, None)
        .expect("display_namelist failed");

    assert!(
        !results.is_empty(),
        "display_namelist should return results"
    );
    assert_results_contain(&results, SEEDED_NAMELIST);
}

#[test]
fn display_seeded_process() {
    let config = load_config();
    let mut session = build_session(&config);

    let results = session
        .display_process(Some(SEEDED_PROCESS), None, None, None)
        .expect("display_process failed");

    assert!(!results.is_empty(), "display_process should return results");
    assert_results_contain(&results, SEEDED_PROCESS);
}

// ---------------------------------------------------------------------------
// Lifecycle tests
// ---------------------------------------------------------------------------

#[test]
fn lifecycle_qlocal() {
    let config = load_config();
    let mut session = build_session(&config);

    // Pre-cleanup.
    let _ = session.delete_qlocal(TEST_QLOCAL, None, None);

    // Define.
    let define_params = params(&[
        ("replace", json!("yes")),
        ("default_persistence", json!("yes")),
        ("description", json!("dev test qlocal")),
    ]);
    session
        .define_qlocal(TEST_QLOCAL, Some(&define_params), None)
        .expect("define_qlocal failed");

    // Display and verify.
    let results = session
        .display_queue(Some(TEST_QLOCAL), None, None, None)
        .expect("display_queue failed");
    assert_results_contain(&results, TEST_QLOCAL);

    // Delete.
    session
        .delete_qlocal(TEST_QLOCAL, None, None)
        .expect("delete_qlocal failed");

    // Verify gone.
    verify_object_gone(&mut session, TEST_QLOCAL, |s, n| {
        s.display_queue(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_qremote() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_qremote(TEST_QREMOTE, None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("remote_queue_name", json!("DEV.TARGET")),
        ("remote_queue_manager_name", json!(config.qmgr_name)),
        ("transmission_queue_name", json!("DEV.XMITQ")),
        ("description", json!("dev test qremote")),
    ]);
    session
        .define_qremote(TEST_QREMOTE, Some(&define_params), None)
        .expect("define_qremote failed");

    let results = session
        .display_queue(Some(TEST_QREMOTE), None, None, None)
        .expect("display_queue failed");
    assert_results_contain(&results, TEST_QREMOTE);

    session
        .delete_qremote(TEST_QREMOTE, None, None)
        .expect("delete_qremote failed");

    verify_object_gone(&mut session, TEST_QREMOTE, |s, n| {
        s.display_queue(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_qalias() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_qalias(TEST_QALIAS, None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("target_queue_name", json!("DEV.QLOCAL")),
        ("description", json!("dev test qalias")),
    ]);
    session
        .define_qalias(TEST_QALIAS, Some(&define_params), None)
        .expect("define_qalias failed");

    let results = session
        .display_queue(Some(TEST_QALIAS), None, None, None)
        .expect("display_queue failed");
    assert_results_contain(&results, TEST_QALIAS);

    session
        .delete_qalias(TEST_QALIAS, None, None)
        .expect("delete_qalias failed");

    verify_object_gone(&mut session, TEST_QALIAS, |s, n| {
        s.display_queue(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_qmodel() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_qmodel(TEST_QMODEL, None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("definition_type", json!("TEMPDYN")),
        ("default_input_open_option", json!("SHARED")),
        ("description", json!("dev test qmodel")),
    ]);
    session
        .define_qmodel(TEST_QMODEL, Some(&define_params), None)
        .expect("define_qmodel failed");

    let results = session
        .display_queue(Some(TEST_QMODEL), None, None, None)
        .expect("display_queue failed");
    assert_results_contain(&results, TEST_QMODEL);

    session
        .delete_qmodel(TEST_QMODEL, None, None)
        .expect("delete_qmodel failed");

    verify_object_gone(&mut session, TEST_QMODEL, |s, n| {
        s.display_queue(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_channel() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_channel(TEST_CHANNEL, None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("channel_type", json!("SVRCONN")),
        ("transport_type", json!("TCP")),
        ("description", json!("dev test channel")),
    ]);
    session
        .define_channel(TEST_CHANNEL, Some(&define_params), None)
        .expect("define_channel failed");

    let results = session
        .display_channel(Some(TEST_CHANNEL), None, None, None)
        .expect("display_channel failed");
    assert_results_contain(&results, TEST_CHANNEL);

    // Alter.
    let alter_params = params(&[
        ("channel_type", json!("SVRCONN")),
        ("description", json!("dev test channel updated")),
    ]);
    session
        .alter_channel(Some(TEST_CHANNEL), Some(&alter_params), None)
        .expect("alter_channel failed");

    let results = session
        .display_channel(Some(TEST_CHANNEL), None, None, None)
        .expect("display_channel failed after alter");
    let matched = results
        .iter()
        .find(|r| contains_string_value(r, TEST_CHANNEL))
        .expect("channel should still exist after alter");
    let description = get_attribute_insensitive(matched, "description")
        .or_else(|| get_attribute_insensitive(matched, "DESCR"));
    assert_eq!(
        description.and_then(Value::as_str),
        Some("dev test channel updated")
    );

    // Delete.
    session
        .delete_channel(TEST_CHANNEL, None, None)
        .expect("delete_channel failed");

    verify_object_gone(&mut session, TEST_CHANNEL, |s, n| {
        s.display_channel(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_listener() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_listener(Some(TEST_LISTENER), None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("transport_type", json!("TCP")),
        ("port", json!(1416)),
        ("start_mode", json!("QMGR")),
        ("description", json!("dev test listener")),
    ]);
    session
        .define_listener(Some(TEST_LISTENER), Some(&define_params), None)
        .expect("define_listener failed");

    let results = session
        .display_listener(Some(TEST_LISTENER), None, None, None)
        .expect("display_listener failed");
    assert_results_contain(&results, TEST_LISTENER);

    // Alter.
    let alter_params = params(&[
        ("transport_type", json!("TCP")),
        ("description", json!("dev test listener updated")),
    ]);
    session
        .alter_listener(Some(TEST_LISTENER), Some(&alter_params), None)
        .expect("alter_listener failed");

    let results = session
        .display_listener(Some(TEST_LISTENER), None, None, None)
        .expect("display_listener failed after alter");
    let matched = results
        .iter()
        .find(|r| contains_string_value(r, TEST_LISTENER))
        .expect("listener should still exist after alter");
    let description = get_attribute_insensitive(matched, "description")
        .or_else(|| get_attribute_insensitive(matched, "DESCR"));
    assert_eq!(
        description.and_then(Value::as_str),
        Some("dev test listener updated")
    );

    // Delete.
    session
        .delete_listener(Some(TEST_LISTENER), None, None)
        .expect("delete_listener failed");

    verify_object_gone(&mut session, TEST_LISTENER, |s, n| {
        s.display_listener(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_process() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_process(Some(TEST_PROCESS), None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("application_id", json!("/bin/true")),
        ("description", json!("dev test process")),
    ]);
    session
        .define_process(Some(TEST_PROCESS), Some(&define_params), None)
        .expect("define_process failed");

    let results = session
        .display_process(Some(TEST_PROCESS), None, None, None)
        .expect("display_process failed");
    assert_results_contain(&results, TEST_PROCESS);

    // Alter.
    let alter_params = params(&[("description", json!("dev test process updated"))]);
    session
        .alter_process(Some(TEST_PROCESS), Some(&alter_params), None)
        .expect("alter_process failed");

    let results = session
        .display_process(Some(TEST_PROCESS), None, None, None)
        .expect("display_process failed after alter");
    let matched = results
        .iter()
        .find(|r| contains_string_value(r, TEST_PROCESS))
        .expect("process should still exist after alter");
    let description = get_attribute_insensitive(matched, "description")
        .or_else(|| get_attribute_insensitive(matched, "DESCR"));
    assert_eq!(
        description.and_then(Value::as_str),
        Some("dev test process updated")
    );

    // Delete.
    session
        .delete_process(Some(TEST_PROCESS), None, None)
        .expect("delete_process failed");

    verify_object_gone(&mut session, TEST_PROCESS, |s, n| {
        s.display_process(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_topic() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_topic(Some(TEST_TOPIC), None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("topic_string", json!("dev/test")),
        ("description", json!("dev test topic")),
    ]);
    session
        .define_topic(Some(TEST_TOPIC), Some(&define_params), None)
        .expect("define_topic failed");

    let results = session
        .display_topic(Some(TEST_TOPIC), None, None, None)
        .expect("display_topic failed");
    assert_results_contain(&results, TEST_TOPIC);

    // Alter.
    let alter_params = params(&[("description", json!("dev test topic updated"))]);
    session
        .alter_topic(Some(TEST_TOPIC), Some(&alter_params), None)
        .expect("alter_topic failed");

    let results = session
        .display_topic(Some(TEST_TOPIC), None, None, None)
        .expect("display_topic failed after alter");
    let matched = results
        .iter()
        .find(|r| contains_string_value(r, TEST_TOPIC))
        .expect("topic should still exist after alter");
    let description = get_attribute_insensitive(matched, "description")
        .or_else(|| get_attribute_insensitive(matched, "DESCR"));
    assert_eq!(
        description.and_then(Value::as_str),
        Some("dev test topic updated")
    );

    // Delete.
    session
        .delete_topic(Some(TEST_TOPIC), None, None)
        .expect("delete_topic failed");

    verify_object_gone(&mut session, TEST_TOPIC, |s, n| {
        s.display_topic(Some(n), None, None, None)
    });
}

#[test]
fn lifecycle_namelist() {
    let config = load_config();
    let mut session = build_session(&config);

    let _ = session.delete_namelist(Some(TEST_NAMELIST), None, None);

    let define_params = params(&[
        ("replace", json!("yes")),
        ("names", json!(["DEV.QLOCAL"])),
        ("description", json!("dev test namelist")),
    ]);
    session
        .define_namelist(Some(TEST_NAMELIST), Some(&define_params), None)
        .expect("define_namelist failed");

    let results = session
        .display_namelist(Some(TEST_NAMELIST), None, None, None)
        .expect("display_namelist failed");
    assert_results_contain(&results, TEST_NAMELIST);

    // Alter.
    let alter_params = params(&[("description", json!("dev test namelist updated"))]);
    session
        .alter_namelist(Some(TEST_NAMELIST), Some(&alter_params), None)
        .expect("alter_namelist failed");

    let results = session
        .display_namelist(Some(TEST_NAMELIST), None, None, None)
        .expect("display_namelist failed after alter");
    let matched = results
        .iter()
        .find(|r| contains_string_value(r, TEST_NAMELIST))
        .expect("namelist should still exist after alter");
    let description = get_attribute_insensitive(matched, "description")
        .or_else(|| get_attribute_insensitive(matched, "DESCR"));
    assert_eq!(
        description.and_then(Value::as_str),
        Some("dev test namelist updated")
    );

    // Delete.
    session
        .delete_namelist(Some(TEST_NAMELIST), None, None)
        .expect("delete_namelist failed");

    verify_object_gone(&mut session, TEST_NAMELIST, |s, n| {
        s.display_namelist(Some(n), None, None, None)
    });
}

// ---------------------------------------------------------------------------
// Ensure lifecycle tests
// ---------------------------------------------------------------------------

#[test]
fn ensure_qmgr_lifecycle() {
    let config = load_config();
    let mut session = build_session(&config);

    // Read current description so we can restore it.
    let qmgr = session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");
    assert!(qmgr.is_some(), "display_qmgr should return an object");
    let qmgr = qmgr.unwrap();
    let original_descr = qmgr
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();

    let test_descr = "dev ensure_qmgr test";

    // Alter to test value.
    let update_params = params(&[("description", json!(test_descr))]);
    let result = session
        .ensure_qmgr(Some(&update_params))
        .expect("ensure_qmgr update failed");
    assert!(
        result.action == EnsureAction::Updated || result.action == EnsureAction::Unchanged,
        "ensure_qmgr should return Updated or Unchanged"
    );

    // Unchanged (same attributes).
    let result = session
        .ensure_qmgr(Some(&update_params))
        .expect("ensure_qmgr unchanged failed");
    assert_eq!(result.action, EnsureAction::Unchanged);

    // Restore original description.
    let restore_params = params(&[("description", json!(original_descr))]);
    session
        .ensure_qmgr(Some(&restore_params))
        .expect("ensure_qmgr restore failed");
}

#[test]
fn ensure_qlocal_lifecycle() {
    let config = load_config();
    let mut session = build_session_non_strict(&config);

    // Pre-cleanup.
    let _ = session.delete_qlocal(TEST_ENSURE_QLOCAL, None, None);

    // Create.
    let create_params = params(&[("description", json!("ensure test"))]);
    let result = session
        .ensure_qlocal(TEST_ENSURE_QLOCAL, Some(&create_params))
        .expect("ensure_qlocal create failed");
    assert_eq!(result.action, EnsureAction::Created);

    // Unchanged (same attributes).
    let result = session
        .ensure_qlocal(TEST_ENSURE_QLOCAL, Some(&create_params))
        .expect("ensure_qlocal unchanged failed");
    assert_eq!(result.action, EnsureAction::Unchanged);

    // Updated (different attribute).
    let update_params = params(&[("description", json!("ensure updated"))]);
    let result = session
        .ensure_qlocal(TEST_ENSURE_QLOCAL, Some(&update_params))
        .expect("ensure_qlocal update failed");
    assert_eq!(result.action, EnsureAction::Updated);

    // Cleanup.
    session
        .delete_qlocal(TEST_ENSURE_QLOCAL, None, None)
        .expect("delete_qlocal cleanup failed");
}

#[test]
fn ensure_channel_lifecycle() {
    let config = load_config();
    let mut session = build_session_non_strict(&config);

    // Pre-cleanup.
    let _ = session.delete_channel(TEST_ENSURE_CHANNEL, None, None);

    // Create.
    let create_params = params(&[
        ("channel_type", json!("SVRCONN")),
        ("description", json!("ensure test")),
    ]);
    let result = session
        .ensure_channel(TEST_ENSURE_CHANNEL, Some(&create_params))
        .expect("ensure_channel create failed");
    assert_eq!(result.action, EnsureAction::Created);

    // Unchanged.
    let result = session
        .ensure_channel(TEST_ENSURE_CHANNEL, Some(&create_params))
        .expect("ensure_channel unchanged failed");
    assert_eq!(result.action, EnsureAction::Unchanged);

    // Updated.
    let update_params = params(&[
        ("channel_type", json!("SVRCONN")),
        ("description", json!("ensure updated")),
    ]);
    let result = session
        .ensure_channel(TEST_ENSURE_CHANNEL, Some(&update_params))
        .expect("ensure_channel update failed");
    assert_eq!(result.action, EnsureAction::Updated);

    // Cleanup.
    session
        .delete_channel(TEST_ENSURE_CHANNEL, None, None)
        .expect("delete_channel cleanup failed");
}

// ---------------------------------------------------------------------------
// LTPA auth
// ---------------------------------------------------------------------------

#[test]
fn ltpa_auth_display_qmgr() {
    let config = load_config();
    let mut session = MqRestSession::builder(
        &config.rest_base_url,
        &config.qmgr_name,
        Credentials::Ltpa {
            username: config.admin_user.clone(),
            password: config.admin_password.clone(),
        },
    )
    .verify_tls(false)
    .build()
    .expect("failed to build LTPA session");

    let result = session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");

    assert!(result.is_some(), "display_qmgr should return an object");
    let qmgr = result.unwrap();
    assert!(
        contains_string_value(&qmgr, &config.qmgr_name),
        "qmgr object should contain the queue manager name"
    );
}

// ---------------------------------------------------------------------------
// Gateway routing tests
// ---------------------------------------------------------------------------

#[test]
fn gateway_display_qmgr_qm2_via_qm1() {
    let config = load_config();
    let mut session = build_gateway_session(
        &config,
        &config.qm2_qmgr_name,
        &config.qmgr_name,
        &config.rest_base_url,
    );

    let result = session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");

    assert!(result.is_some(), "display_qmgr should return an object");
    let qmgr = result.unwrap();
    assert!(
        contains_string_value(&qmgr, &config.qm2_qmgr_name),
        "should see QM2 via QM1 gateway"
    );
}

#[test]
fn gateway_display_qmgr_qm1_via_qm2() {
    let config = load_config();
    let mut session = build_gateway_session(
        &config,
        &config.qmgr_name,
        &config.qm2_qmgr_name,
        &config.qm2_rest_base_url,
    );

    let result = session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");

    assert!(result.is_some(), "display_qmgr should return an object");
    let qmgr = result.unwrap();
    assert!(
        contains_string_value(&qmgr, &config.qmgr_name),
        "should see QM1 via QM2 gateway"
    );
}

#[test]
fn gateway_display_queue_qm2_via_qm1() {
    let config = load_config();
    let mut session = build_gateway_session(
        &config,
        &config.qm2_qmgr_name,
        &config.qmgr_name,
        &config.rest_base_url,
    );

    let results = session
        .display_queue(Some("DEV.QLOCAL"), None, None, None)
        .expect("display_queue failed");

    assert!(!results.is_empty(), "display_queue should return results");
    assert_results_contain(&results, "DEV.QLOCAL");
}

#[test]
fn gateway_session_properties() {
    let config = load_config();
    let session = build_gateway_session(
        &config,
        &config.qm2_qmgr_name,
        &config.qmgr_name,
        &config.rest_base_url,
    );

    assert_eq!(session.qmgr_name(), config.qm2_qmgr_name);
    assert_eq!(session.gateway_qmgr(), Some(config.qmgr_name.as_str()));
}

// ---------------------------------------------------------------------------
// Session state test
// ---------------------------------------------------------------------------

#[test]
fn session_state_populated_after_command() {
    let config = load_config();
    let mut session = build_session(&config);

    session
        .display_qmgr(None, None)
        .expect("display_qmgr failed");

    assert!(
        session.last_http_status.is_some(),
        "last_http_status should be populated"
    );
    assert!(
        session.last_response_text.is_some(),
        "last_response_text should be populated"
    );
    assert!(
        session.last_response_payload.is_some(),
        "last_response_payload should be populated"
    );
    assert!(
        session.last_command_payload.is_some(),
        "last_command_payload should be populated"
    );
}

// ---------------------------------------------------------------------------
// Example function integration tests
// ---------------------------------------------------------------------------

#[cfg(feature = "examples")]
mod example_tests {
    use super::*;
    use mq_rest_admin::examples;

    fn build_qm2_session(config: &IntegrationConfig) -> MqRestSession {
        MqRestSession::builder(
            &config.qm2_rest_base_url,
            &config.qm2_qmgr_name,
            Credentials::Basic {
                username: config.admin_user.clone(),
                password: config.admin_password.clone(),
            },
        )
        .verify_tls(false)
        .build()
        .expect("failed to build QM2 session")
    }

    #[test]
    fn health_check_qm1() {
        let config = load_config();
        let mut session = build_session(&config);

        let result = examples::check_health(&mut session).expect("check_health failed");
        assert!(result.reachable);
        assert!(result.passed);
        assert_eq!(result.qmgr_name, config.qmgr_name);
    }

    #[test]
    fn health_check_qm2() {
        let config = load_config();
        let mut session = build_qm2_session(&config);

        let result = examples::check_health(&mut session).expect("check_health failed");
        assert!(result.reachable);
        assert!(result.passed);
        assert_eq!(result.qmgr_name, config.qm2_qmgr_name);
    }

    #[test]
    fn queue_depth_monitor() {
        let config = load_config();
        let mut session = build_session(&config);

        let results = examples::monitor_queue_depths(&mut session, 80.0).expect("monitor failed");
        assert!(!results.is_empty(), "should return at least one queue");
        assert!(
            results.iter().any(|q| q.name == "DEV.QLOCAL"),
            "should contain DEV.QLOCAL"
        );
    }

    #[test]
    fn channel_status_report() {
        let config = load_config();
        let mut session = build_session(&config);

        let results = examples::report_channel_status(&mut session).expect("channel status failed");
        assert!(!results.is_empty(), "should return at least one channel");
        assert!(
            results.iter().any(|c| c.name == "DEV.SVRCONN"),
            "should contain DEV.SVRCONN"
        );
    }

    #[test]
    fn dlq_inspector() {
        let config = load_config();
        let mut session = build_session(&config);

        let report = examples::inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(report.configured);
        assert_eq!(report.dlq_name, "DEV.DEAD.LETTER");
        assert_eq!(report.current_depth, 0);
    }

    #[test]
    fn queue_status_handles() {
        let config = load_config();
        let mut session = build_session(&config);

        let _results = examples::report_queue_handles(&mut session).expect("queue handles failed");
        // May be empty if no handles are open — just assert it succeeds
    }

    #[test]
    fn connection_handles() {
        let config = load_config();
        let mut session = build_session(&config);

        let _results =
            examples::report_connection_handles(&mut session).expect("connection handles failed");
        // May be empty — just assert it succeeds
    }

    #[test]
    fn provision_and_teardown() {
        let config = load_config();
        let mut qm1 = build_session(&config);
        let mut qm2 = build_qm2_session(&config);

        let result = examples::provision(&mut qm1, &mut qm2).expect("provision failed");
        assert!(
            !result.objects_created.is_empty(),
            "should create at least one object"
        );
        assert!(result.verified, "verification should pass");

        let failures = examples::teardown(&mut qm1, &mut qm2).expect("teardown failed");
        assert!(failures.is_empty(), "teardown should have no failures");
    }
}
