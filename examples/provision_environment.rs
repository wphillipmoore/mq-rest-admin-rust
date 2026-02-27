//! Environment provisioner.
//!
//! Defines a complete set of queues, channels, and remote queue
//! definitions across two queue managers, then verifies connectivity.
//! Includes teardown to remove all provisioned objects.
//!
//! ```text
//! cargo run --example provision_environment
//! ```
//!
//! Requires both QM1 and QM2 to be running. Set `MQ_REST_BASE_URL_QM2`
//! to the QM2 REST endpoint (default: `https://localhost:9444/ibmmq/rest/v2`).

use std::collections::HashMap;
use std::env;

use mq_rest_admin::{Credentials, MqRestSession};
use serde_json::Value;

const PREFIX: &str = "PROV";

fn define(
    session: &mut MqRestSession,
    method: &str,
    name: &str,
    params: &HashMap<String, Value>,
    created: &mut Vec<String>,
    failed: &mut Vec<String>,
) {
    let label = format!("{}/{name}", session.qmgr_name());
    let result = match method {
        "define_qlocal" => session.define_qlocal(name, Some(params), None),
        "define_qremote" => session.define_qremote(name, Some(params), None),
        "define_channel" => session.define_channel(name, Some(params), None),
        _ => unreachable!(),
    };
    match result {
        Ok(()) => created.push(label),
        Err(_) => failed.push(label),
    }
}

fn delete(
    session: &mut MqRestSession,
    method: &str,
    name: &str,
    label: &str,
    failures: &mut Vec<String>,
) {
    let result = match method {
        "delete_queue" => session.delete_queue(name, None, None),
        "delete_channel" => session.delete_channel(name, None, None),
        _ => unreachable!(),
    };
    if result.is_err() {
        failures.push(format!("{label}/{name}"));
    }
}

fn params(entries: &[(&str, &str)]) -> HashMap<String, Value> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), Value::String((*v).to_string())))
        .collect()
}

fn define_queues(
    qm1: &mut MqRestSession,
    qm2: &mut MqRestSession,
    created: &mut Vec<String>,
    failed: &mut Vec<String>,
) {
    // Local queues
    define(
        qm1,
        "define_qlocal",
        &format!("{PREFIX}.QM1.LOCAL"),
        &params(&[
            ("replace", "yes"),
            ("default_persistence", "yes"),
            ("description", "provisioned local queue on QM1"),
        ]),
        created,
        failed,
    );
    define(
        qm2,
        "define_qlocal",
        &format!("{PREFIX}.QM2.LOCAL"),
        &params(&[
            ("replace", "yes"),
            ("default_persistence", "yes"),
            ("description", "provisioned local queue on QM2"),
        ]),
        created,
        failed,
    );

    // Transmission queues
    define(
        qm1,
        "define_qlocal",
        &format!("{PREFIX}.QM1.TO.QM2.XMITQ"),
        &params(&[
            ("replace", "yes"),
            ("usage", "XMITQ"),
            ("description", "xmit queue QM1 to QM2"),
        ]),
        created,
        failed,
    );
    define(
        qm2,
        "define_qlocal",
        &format!("{PREFIX}.QM2.TO.QM1.XMITQ"),
        &params(&[
            ("replace", "yes"),
            ("usage", "XMITQ"),
            ("description", "xmit queue QM2 to QM1"),
        ]),
        created,
        failed,
    );

    // Remote queues
    define(
        qm1,
        "define_qremote",
        &format!("{PREFIX}.REMOTE.TO.QM2"),
        &params(&[
            ("replace", "yes"),
            ("remote_queue_name", &format!("{PREFIX}.QM2.LOCAL")),
            ("remote_queue_manager_name", "QM2"),
            (
                "transmission_queue_name",
                &format!("{PREFIX}.QM1.TO.QM2.XMITQ"),
            ),
            ("description", "remote queue QM1 to QM2"),
        ]),
        created,
        failed,
    );
    define(
        qm2,
        "define_qremote",
        &format!("{PREFIX}.REMOTE.TO.QM1"),
        &params(&[
            ("replace", "yes"),
            ("remote_queue_name", &format!("{PREFIX}.QM1.LOCAL")),
            ("remote_queue_manager_name", "QM1"),
            (
                "transmission_queue_name",
                &format!("{PREFIX}.QM2.TO.QM1.XMITQ"),
            ),
            ("description", "remote queue QM2 to QM1"),
        ]),
        created,
        failed,
    );
}

fn define_channels(
    qm1: &mut MqRestSession,
    qm2: &mut MqRestSession,
    created: &mut Vec<String>,
    failed: &mut Vec<String>,
) {
    define(
        qm1,
        "define_channel",
        &format!("{PREFIX}.QM1.TO.QM2"),
        &params(&[
            ("replace", "yes"),
            ("channel_type", "SDR"),
            ("transport_type", "TCP"),
            ("connection_name", "qm2(1414)"),
            (
                "transmission_queue_name",
                &format!("{PREFIX}.QM1.TO.QM2.XMITQ"),
            ),
            ("description", "sender QM1 to QM2"),
        ]),
        created,
        failed,
    );
    define(
        qm2,
        "define_channel",
        &format!("{PREFIX}.QM1.TO.QM2"),
        &params(&[
            ("replace", "yes"),
            ("channel_type", "RCVR"),
            ("transport_type", "TCP"),
            ("description", "receiver QM1 to QM2"),
        ]),
        created,
        failed,
    );
    define(
        qm2,
        "define_channel",
        &format!("{PREFIX}.QM2.TO.QM1"),
        &params(&[
            ("replace", "yes"),
            ("channel_type", "SDR"),
            ("transport_type", "TCP"),
            ("connection_name", "qm1(1414)"),
            (
                "transmission_queue_name",
                &format!("{PREFIX}.QM2.TO.QM1.XMITQ"),
            ),
            ("description", "sender QM2 to QM1"),
        ]),
        created,
        failed,
    );
    define(
        qm1,
        "define_channel",
        &format!("{PREFIX}.QM2.TO.QM1"),
        &params(&[
            ("replace", "yes"),
            ("channel_type", "RCVR"),
            ("transport_type", "TCP"),
            ("description", "receiver QM2 to QM1"),
        ]),
        created,
        failed,
    );
}

fn provision(qm1: &mut MqRestSession, qm2: &mut MqRestSession) -> (Vec<String>, Vec<String>, bool) {
    let mut created: Vec<String> = Vec::new();
    let mut failed: Vec<String> = Vec::new();

    define_queues(qm1, qm2, &mut created, &mut failed);
    define_channels(qm1, qm2, &mut created, &mut failed);

    // Verify
    let verified = match (
        qm1.display_queue(Some(&format!("{PREFIX}.*")), None, None, None),
        qm2.display_queue(Some(&format!("{PREFIX}.*")), None, None, None),
    ) {
        (Ok(q1), Ok(q2)) => q1.len() >= 3 && q2.len() >= 3,
        _ => false,
    };

    (created, failed, verified)
}

fn teardown(qm1: &mut MqRestSession, qm2: &mut MqRestSession) -> Vec<String> {
    let mut failures: Vec<String> = Vec::new();

    let channels = [
        format!("{PREFIX}.QM1.TO.QM2"),
        format!("{PREFIX}.QM2.TO.QM1"),
    ];
    let queues = [
        format!("{PREFIX}.REMOTE.TO.QM1"),
        format!("{PREFIX}.REMOTE.TO.QM2"),
        format!("{PREFIX}.QM1.TO.QM2.XMITQ"),
        format!("{PREFIX}.QM2.TO.QM1.XMITQ"),
        format!("{PREFIX}.QM1.LOCAL"),
        format!("{PREFIX}.QM2.LOCAL"),
    ];

    for (session, label) in [(&mut *qm1, "QM1"), (&mut *qm2, "QM2")] {
        for channel in &channels {
            delete(session, "delete_channel", channel, label, &mut failures);
        }
        for queue in &queues {
            delete(session, "delete_queue", queue, label, &mut failures);
        }
    }

    failures
}

fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} environment variable is required"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = require_env("MQ_ADMIN_USER");
    let password = require_env("MQ_ADMIN_PASSWORD");
    let qm1_url = require_env("MQ_REST_BASE_URL");
    let qm2_url = require_env("MQ_REST_BASE_URL_QM2");

    let mut qm1 = MqRestSession::builder(
        &qm1_url,
        "QM1",
        Credentials::Ltpa {
            username: username.clone(),
            password: password.clone(),
        },
    )
    .verify_tls(false)
    .build()?;

    let mut qm2 = MqRestSession::builder(&qm2_url, "QM2", Credentials::Ltpa { username, password })
        .verify_tls(false)
        .build()?;

    println!("\n=== Provisioning environment ===");
    let (created, failed, verified) = provision(&mut qm1, &mut qm2);

    println!("\nCreated: {}", created.len());
    for obj in &created {
        println!("  + {obj}");
    }
    if !failed.is_empty() {
        println!("\nFailed: {}", failed.len());
        for obj in &failed {
            println!("  ! {obj}");
        }
    }
    println!("\nVerified: {verified}");

    println!("\n=== Tearing down ===");
    let teardown_failures = teardown(&mut qm1, &mut qm2);
    if teardown_failures.is_empty() {
        println!("Teardown complete.");
    } else {
        println!("Teardown failures: {teardown_failures:?}");
    }

    Ok(())
}
