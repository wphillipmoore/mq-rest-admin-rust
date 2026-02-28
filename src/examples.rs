//! Importable example functions for the MQ REST admin library.
//!
//! This module provides the logic behind the `examples/*.rs` binaries as
//! public, testable functions.  Each function accepts an `&mut MqRestSession`
//! and returns a typed result struct.
//!
//! Enable with the `examples` Cargo feature:
//!
//! ```text
//! cargo test --features examples
//! cargo run  --features examples --example health_check
//! ```

use std::collections::HashMap;

use serde_json::Value;

use crate::error::Result;
use crate::session::MqRestSession;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn get_str(map: &HashMap<String, Value>, key: &str) -> String {
    map.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string()
}

fn get_i64(map: &HashMap<String, Value>, key: &str) -> i64 {
    match map.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Result structs
// ---------------------------------------------------------------------------

/// Health check result for a single queue manager.
pub struct HealthCheckResult {
    /// Whether the queue manager REST endpoint is reachable.
    pub reachable: bool,
    /// Whether all health checks passed.
    pub passed: bool,
    /// Queue manager name as reported by the endpoint.
    pub qmgr_name: String,
    /// HA status string (e.g. `"RUNNING"`).
    pub status: String,
    /// Command server status (e.g. `"RUNNING"`).
    pub command_server: String,
    /// Listener definitions with their start modes.
    pub listeners: Vec<ListenerInfo>,
}

/// A single listener returned by the health check.
pub struct ListenerInfo {
    /// Listener name.
    pub name: String,
    /// Start mode (e.g. `"QMGR"`, `"MANUAL"`).
    pub start_mode: String,
}

/// Depth information for a single local queue.
pub struct QueueDepthInfo {
    /// Queue name.
    pub name: String,
    /// Current message depth.
    pub current_depth: i64,
    /// Maximum queue depth.
    pub max_depth: i64,
    /// Depth as a percentage of maximum.
    pub depth_pct: f64,
    /// Number of open-for-input handles.
    pub open_input: i64,
    /// Number of open-for-output handles.
    pub open_output: i64,
    /// Whether the depth exceeds the threshold.
    pub warning: bool,
}

/// Channel definition merged with live status.
pub struct ChannelInfo {
    /// Channel name.
    pub name: String,
    /// Channel type (e.g. `"SDR"`, `"RCVR"`, `"SVRCONN"`).
    pub channel_type: String,
    /// Connection name from the definition.
    pub connection_name: String,
    /// Whether a definition exists for this channel.
    pub defined: bool,
    /// Live status or `"INACTIVE"` if not running.
    pub status: String,
}

/// Dead letter queue inspection report.
pub struct DlqReport {
    /// Whether a DLQ is configured on the queue manager.
    pub configured: bool,
    /// DLQ name (empty if not configured).
    pub dlq_name: String,
    /// Current message depth.
    pub current_depth: i64,
    /// Maximum queue depth.
    pub max_depth: i64,
    /// Depth as a percentage of maximum.
    pub depth_pct: f64,
    /// Number of open-for-input handles.
    pub open_input: i64,
    /// Number of open-for-output handles.
    pub open_output: i64,
    /// Actionable suggestion based on depth.
    pub suggestion: String,
}

/// A single queue handle entry from `DISPLAY QSTATUS TYPE(HANDLE)`.
pub struct QueueHandleInfo {
    /// Queue name.
    pub queue_name: String,
    /// Handle state.
    pub handle_state: String,
    /// Connection identifier.
    pub connection_id: String,
    /// Open options.
    pub open_options: String,
}

/// A single connection handle entry from `DISPLAY CONN TYPE(HANDLE)`.
pub struct ConnectionHandleInfo {
    /// Connection identifier.
    pub connection_id: String,
    /// Object name.
    pub object_name: String,
    /// Handle state.
    pub handle_state: String,
    /// Object type.
    pub object_type: String,
}

/// Result of provisioning objects across two queue managers.
pub struct ProvisionResult {
    /// Successfully created objects (`"QM/NAME"` format).
    pub objects_created: Vec<String>,
    /// Objects that failed to create.
    pub failures: Vec<String>,
    /// Whether post-provision verification passed.
    pub verified: bool,
}

// ---------------------------------------------------------------------------
// Public functions
// ---------------------------------------------------------------------------

/// Perform a health check on a queue manager.
///
/// Checks QMGR status, command server availability, and listener state.
pub fn check_health(session: &mut MqRestSession) -> Result<HealthCheckResult> {
    let mut status = "UNKNOWN".to_string();
    let mut command_server = "UNKNOWN".to_string();
    let mut listeners: Vec<ListenerInfo> = Vec::new();
    let qmgr_name = session.qmgr_name().to_string();

    let Ok(_qmgr) = session.display_qmgr(None, None) else {
        return Ok(HealthCheckResult {
            reachable: false,
            passed: false,
            qmgr_name,
            status,
            command_server,
            listeners,
        });
    };

    if let Ok(Some(qs)) = session.display_qmstatus(None, None) {
        status = get_str(&qs, "ha_status");
    }

    if let Ok(Some(cs)) = session.display_cmdserv(None, None) {
        command_server = get_str(&cs, "status");
    }

    if let Ok(lst) = session.display_listener(Some("*"), None, None, None) {
        for listener in lst {
            listeners.push(ListenerInfo {
                name: get_str(&listener, "listener_name"),
                start_mode: get_str(&listener, "start_mode"),
            });
        }
    }

    let passed = status != "UNKNOWN";
    Ok(HealthCheckResult {
        reachable: true,
        passed,
        qmgr_name,
        status,
        command_server,
        listeners,
    })
}

/// Monitor local queue depths and flag queues above the given threshold.
///
/// Returns queue depth information sorted by depth percentage descending.
pub fn monitor_queue_depths(
    session: &mut MqRestSession,
    threshold_pct: f64,
) -> Result<Vec<QueueDepthInfo>> {
    let queues = session.display_queue(Some("*"), None, None, None)?;
    let mut results: Vec<QueueDepthInfo> = Vec::new();

    for queue in &queues {
        let qtype = get_str(queue, "type").to_uppercase();
        if qtype != "QLOCAL" && qtype != "LOCAL" {
            continue;
        }

        let current_depth = get_i64(queue, "current_queue_depth");
        let max_depth = get_i64(queue, "max_queue_depth");

        #[allow(clippy::cast_precision_loss)]
        let depth_pct = if max_depth > 0 {
            current_depth as f64 / max_depth as f64 * 100.0
        } else {
            0.0
        };

        results.push(QueueDepthInfo {
            name: get_str(queue, "queue_name"),
            current_depth,
            max_depth,
            depth_pct,
            open_input: get_i64(queue, "open_input_count"),
            open_output: get_i64(queue, "open_output_count"),
            warning: depth_pct >= threshold_pct,
        });
    }

    results.sort_by(|a, b| b.depth_pct.total_cmp(&a.depth_pct));
    Ok(results)
}

/// Report channel definitions merged with live channel status.
pub fn report_channel_status(session: &mut MqRestSession) -> Result<Vec<ChannelInfo>> {
    let channels = session.display_channel(Some("*"), None, None, None)?;
    let mut definitions: HashMap<String, HashMap<String, Value>> = HashMap::new();
    for channel in channels {
        let cname = get_str(&channel, "channel_name");
        if !cname.is_empty() {
            definitions.insert(cname, channel);
        }
    }

    let mut live_status: HashMap<String, String> = HashMap::new();
    if let Ok(statuses) = session.display_chstatus(Some("*"), None, None, None) {
        for entry in statuses {
            let cname = get_str(&entry, "channel_name");
            let cstatus = get_str(&entry, "status");
            if !cname.is_empty() {
                live_status.insert(cname, cstatus);
            }
        }
    }

    let mut results: Vec<ChannelInfo> = Vec::new();

    let mut def_names: Vec<&String> = definitions.keys().collect();
    def_names.sort();
    for cname in def_names {
        let defn = &definitions[cname];
        let ctype = get_str(defn, "channel_type");
        let conname = get_str(defn, "connection_name");
        let status = live_status
            .get(cname)
            .cloned()
            .unwrap_or_else(|| "INACTIVE".into());

        results.push(ChannelInfo {
            name: cname.clone(),
            channel_type: ctype,
            connection_name: conname,
            defined: true,
            status,
        });
    }

    let mut status_names: Vec<&String> = live_status.keys().collect();
    status_names.sort();
    for cname in status_names {
        if !definitions.contains_key(cname) {
            results.push(ChannelInfo {
                name: cname.clone(),
                channel_type: String::new(),
                connection_name: String::new(),
                defined: false,
                status: live_status[cname].clone(),
            });
        }
    }

    Ok(results)
}

const CRITICAL_DEPTH_PCT: f64 = 90.0;

/// Inspect the dead letter queue configuration and depth.
pub fn inspect_dlq(session: &mut MqRestSession) -> Result<DlqReport> {
    let qmgr = session.display_qmgr(None, None)?;

    let dlq_name = qmgr
        .as_ref()
        .map(|q| get_str(q, "dead_letter_queue_name"))
        .unwrap_or_default();

    if dlq_name.is_empty() {
        return Ok(DlqReport {
            configured: false,
            dlq_name: String::new(),
            current_depth: 0,
            max_depth: 0,
            depth_pct: 0.0,
            open_input: 0,
            open_output: 0,
            suggestion: "No dead letter queue configured. Define one with ALTER QMGR DEADQ."
                .to_string(),
        });
    }

    let queues = session.display_queue(Some(&dlq_name), None, None, None)?;
    if queues.is_empty() {
        return Ok(DlqReport {
            configured: true,
            dlq_name: dlq_name.clone(),
            current_depth: 0,
            max_depth: 0,
            depth_pct: 0.0,
            open_input: 0,
            open_output: 0,
            suggestion: format!("DLQ '{dlq_name}' is configured but the queue does not exist."),
        });
    }

    let dlq = &queues[0];
    let current_depth = get_i64(dlq, "current_queue_depth");
    let max_depth = get_i64(dlq, "max_queue_depth");
    let open_input = get_i64(dlq, "open_input_count");
    let open_output = get_i64(dlq, "open_output_count");

    #[allow(clippy::cast_precision_loss)]
    let depth_pct = if max_depth > 0 {
        current_depth as f64 / max_depth as f64 * 100.0
    } else {
        0.0
    };

    let suggestion = if current_depth == 0 {
        "DLQ is empty. No action needed."
    } else if depth_pct >= CRITICAL_DEPTH_PCT {
        "DLQ is near capacity. Investigate and clear undeliverable messages urgently."
    } else {
        "DLQ has messages. Investigate undeliverable messages."
    };

    Ok(DlqReport {
        configured: true,
        dlq_name,
        current_depth,
        max_depth,
        depth_pct,
        open_input,
        open_output,
        suggestion: suggestion.to_string(),
    })
}

/// Report queue handles from `DISPLAY QSTATUS TYPE(HANDLE)`.
pub fn report_queue_handles(session: &mut MqRestSession) -> Result<Vec<QueueHandleInfo>> {
    let mut qstatus_params: HashMap<String, Value> = HashMap::new();
    qstatus_params.insert("type".into(), Value::String("HANDLE".into()));

    let entries = session
        .display_qstatus(Some("*"), Some(&qstatus_params), None, None)
        .unwrap_or_default();

    let results = entries
        .iter()
        .map(|entry| QueueHandleInfo {
            queue_name: get_str(entry, "queue_name"),
            handle_state: get_str(entry, "handle_state"),
            connection_id: get_str(entry, "connection_id"),
            open_options: get_str(entry, "open_options"),
        })
        .collect();

    Ok(results)
}

/// Report connection handles from `DISPLAY CONN TYPE(HANDLE)`.
pub fn report_connection_handles(session: &mut MqRestSession) -> Result<Vec<ConnectionHandleInfo>> {
    let mut conn_params: HashMap<String, Value> = HashMap::new();
    conn_params.insert(
        "connection_info_type".into(),
        Value::String("HANDLE".into()),
    );

    let entries = session
        .display_conn(Some("*"), Some(&conn_params), None, None)
        .unwrap_or_default();

    let results = entries
        .iter()
        .map(|entry| ConnectionHandleInfo {
            connection_id: get_str(entry, "connection_id"),
            object_name: get_str(entry, "object_name"),
            handle_state: get_str(entry, "handle_state"),
            object_type: get_str(entry, "object_type"),
        })
        .collect();

    Ok(results)
}

// ---------------------------------------------------------------------------
// Provision helpers
// ---------------------------------------------------------------------------

const PREFIX: &str = "PROV";

enum DefineMethod {
    Qlocal,
    Qremote,
    Channel,
}

fn define(
    session: &mut MqRestSession,
    method: DefineMethod,
    name: &str,
    params: &HashMap<String, Value>,
    created: &mut Vec<String>,
    failed: &mut Vec<String>,
) {
    let label = format!("{}/{name}", session.qmgr_name());
    let result = match method {
        DefineMethod::Qlocal => session.define_qlocal(name, Some(params), None),
        DefineMethod::Qremote => session.define_qremote(name, Some(params), None),
        DefineMethod::Channel => session.define_channel(name, Some(params), None),
    };
    match result {
        Ok(()) => created.push(label),
        Err(_) => failed.push(label),
    }
}

enum DeleteMethod {
    Queue,
    Channel,
}

fn delete(
    session: &mut MqRestSession,
    method: DeleteMethod,
    name: &str,
    label: &str,
    failures: &mut Vec<String>,
) {
    let result = match method {
        DeleteMethod::Queue => session.delete_queue(name, None, None),
        DeleteMethod::Channel => session.delete_channel(name, None, None),
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
    define(
        qm1,
        DefineMethod::Qlocal,
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
        DefineMethod::Qlocal,
        &format!("{PREFIX}.QM2.LOCAL"),
        &params(&[
            ("replace", "yes"),
            ("default_persistence", "yes"),
            ("description", "provisioned local queue on QM2"),
        ]),
        created,
        failed,
    );

    define(
        qm1,
        DefineMethod::Qlocal,
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
        DefineMethod::Qlocal,
        &format!("{PREFIX}.QM2.TO.QM1.XMITQ"),
        &params(&[
            ("replace", "yes"),
            ("usage", "XMITQ"),
            ("description", "xmit queue QM2 to QM1"),
        ]),
        created,
        failed,
    );

    define(
        qm1,
        DefineMethod::Qremote,
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
        DefineMethod::Qremote,
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
        DefineMethod::Channel,
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
        DefineMethod::Channel,
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
        DefineMethod::Channel,
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
        DefineMethod::Channel,
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

/// Provision queues, channels, and remote queue definitions across two
/// queue managers.
pub fn provision(qm1: &mut MqRestSession, qm2: &mut MqRestSession) -> Result<ProvisionResult> {
    let mut created: Vec<String> = Vec::new();
    let mut failed: Vec<String> = Vec::new();

    define_queues(qm1, qm2, &mut created, &mut failed);
    define_channels(qm1, qm2, &mut created, &mut failed);

    let verified = match (
        qm1.display_queue(Some(&format!("{PREFIX}.*")), None, None, None),
        qm2.display_queue(Some(&format!("{PREFIX}.*")), None, None, None),
    ) {
        (Ok(q1), Ok(q2)) => q1.len() >= 3 && q2.len() >= 3,
        _ => false,
    };

    Ok(ProvisionResult {
        objects_created: created,
        failures: failed,
        verified,
    })
}

/// Tear down all provisioned objects from both queue managers.
pub fn teardown(qm1: &mut MqRestSession, qm2: &mut MqRestSession) -> Result<Vec<String>> {
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
            delete(
                session,
                DeleteMethod::Channel,
                channel,
                label,
                &mut failures,
            );
        }
        for queue in &queues {
            delete(session, DeleteMethod::Queue, queue, label, &mut failures);
        }
    }

    Ok(failures)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use crate::test_helpers::{
        MockTransport, empty_success_response, mock_session, success_response,
    };

    use super::*;

    // Helpers ---------------------------------------------------------------

    fn qmgr_response(name: &str) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("queue_manager_name".into(), json!(name));
        m
    }

    fn qmstatus_response() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("ha_status".into(), json!("RUNNING"));
        m
    }

    fn cmdserv_response() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("status".into(), json!("RUNNING"));
        m
    }

    fn listener_response(name: &str, mode: &str) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("listener_name".into(), json!(name));
        m.insert("start_mode".into(), json!(mode));
        m
    }

    fn local_queue_response(
        name: &str,
        depth: i64,
        max: i64,
        open_in: i64,
        open_out: i64,
    ) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("queue_name".into(), json!(name));
        m.insert("type".into(), json!("QLOCAL"));
        m.insert("current_queue_depth".into(), json!(depth));
        m.insert("max_queue_depth".into(), json!(max));
        m.insert("open_input_count".into(), json!(open_in));
        m.insert("open_output_count".into(), json!(open_out));
        m
    }

    fn channel_def_response(name: &str, ctype: &str, conn: &str) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("channel_name".into(), json!(name));
        m.insert("channel_type".into(), json!(ctype));
        m.insert("connection_name".into(), json!(conn));
        m
    }

    fn chstatus_response(name: &str, status: &str) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("channel_name".into(), json!(name));
        m.insert("status".into(), json!(status));
        m
    }

    fn qmgr_with_dlq(dlq_name: &str) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("dead_letter_queue_name".into(), json!(dlq_name));
        m
    }

    fn queue_handle_response(
        queue: &str,
        state: &str,
        conn: &str,
        opts: &str,
    ) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("queue_name".into(), json!(queue));
        m.insert("handle_state".into(), json!(state));
        m.insert("connection_id".into(), json!(conn));
        m.insert("open_options".into(), json!(opts));
        m
    }

    fn conn_handle_response(
        conn: &str,
        obj: &str,
        state: &str,
        otype: &str,
    ) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("connection_id".into(), json!(conn));
        m.insert("object_name".into(), json!(obj));
        m.insert("handle_state".into(), json!(state));
        m.insert("object_type".into(), json!(otype));
        m
    }

    // health_check ----------------------------------------------------------

    #[test]
    fn health_check_passing() {
        let transport = MockTransport::new(vec![
            // display_qmgr
            success_response(vec![qmgr_response("QM1")]),
            // display_qmstatus
            success_response(vec![qmstatus_response()]),
            // display_cmdserv
            success_response(vec![cmdserv_response()]),
            // display_listener
            success_response(vec![listener_response("LISTENER.TCP", "QMGR")]),
        ]);
        let mut session = mock_session(transport);

        let result = check_health(&mut session).expect("check_health failed");
        assert!(result.reachable);
        assert!(result.passed);
        assert_eq!(result.qmgr_name, "QM1");
        assert_eq!(result.status, "RUNNING");
        assert_eq!(result.command_server, "RUNNING");
        assert_eq!(result.listeners.len(), 1);
        assert_eq!(result.listeners[0].name, "LISTENER.TCP");
        assert_eq!(result.listeners[0].start_mode, "QMGR");
    }

    #[test]
    fn health_check_unreachable() {
        let transport = MockTransport::new(vec![
            // display_qmgr fails (command error)
            crate::test_helpers::error_response(2, 3008),
        ]);
        let mut session = mock_session(transport);

        let result = check_health(&mut session).expect("check_health failed");
        assert!(!result.reachable);
        assert!(!result.passed);
        assert_eq!(result.status, "UNKNOWN");
        assert_eq!(result.command_server, "UNKNOWN");
        assert!(result.listeners.is_empty());
    }

    #[test]
    fn health_check_listener_error() {
        let transport = MockTransport::new(vec![
            // display_qmgr succeeds
            success_response(vec![qmgr_response("QM1")]),
            // display_qmstatus
            success_response(vec![qmstatus_response()]),
            // display_cmdserv
            success_response(vec![cmdserv_response()]),
            // display_listener fails
            crate::test_helpers::error_response(2, 3065),
        ]);
        let mut session = mock_session(transport);

        let result = check_health(&mut session).expect("check_health failed");
        assert!(result.reachable);
        assert!(result.passed);
        assert!(result.listeners.is_empty());
    }

    #[test]
    fn health_check_no_status_or_cmdserv() {
        let transport = MockTransport::new(vec![
            // display_qmgr succeeds
            success_response(vec![qmgr_response("QM1")]),
            // display_qmstatus returns empty
            empty_success_response(),
            // display_cmdserv returns empty
            empty_success_response(),
            // display_listener returns empty
            empty_success_response(),
        ]);
        let mut session = mock_session(transport);

        let result = check_health(&mut session).expect("check_health failed");
        assert!(result.reachable);
        assert!(!result.passed); // status stays "UNKNOWN"
        assert_eq!(result.status, "UNKNOWN");
        assert_eq!(result.command_server, "UNKNOWN");
        assert!(result.listeners.is_empty());
    }

    // monitor_queue_depths --------------------------------------------------

    #[test]
    fn monitor_queue_depths_basic() {
        let transport = MockTransport::new(vec![success_response(vec![
            local_queue_response("Q1", 50, 100, 1, 2),
            local_queue_response("Q2", 90, 100, 0, 0),
            // Non-local queue should be filtered out
            {
                let mut m = HashMap::new();
                m.insert("queue_name".into(), json!("REMOTE.Q"));
                m.insert("type".into(), json!("QREMOTE"));
                m
            },
        ])]);
        let mut session = mock_session(transport);

        let results =
            monitor_queue_depths(&mut session, 80.0).expect("monitor_queue_depths failed");
        assert_eq!(results.len(), 2);
        // Sorted by depth_pct descending
        assert_eq!(results[0].name, "Q2");
        assert!(results[0].warning);
        assert_eq!(results[1].name, "Q1");
        assert!(!results[1].warning);
    }

    #[test]
    fn monitor_queue_depths_zero_max() {
        let transport = MockTransport::new(vec![success_response(vec![local_queue_response(
            "Q1", 0, 0, 0, 0,
        )])]);
        let mut session = mock_session(transport);

        let results =
            monitor_queue_depths(&mut session, 80.0).expect("monitor_queue_depths failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].depth_pct, 0.0);
        assert!(!results[0].warning);
    }

    // report_channel_status -------------------------------------------------

    #[test]
    fn channel_status_with_definitions_and_status() {
        let transport = MockTransport::new(vec![
            // display_channel
            success_response(vec![
                channel_def_response("CHL.A", "SDR", "host(1414)"),
                channel_def_response("CHL.B", "RCVR", ""),
            ]),
            // display_chstatus
            success_response(vec![
                chstatus_response("CHL.A", "RUNNING"),
                chstatus_response("CHL.ORPHAN", "STOPPED"),
            ]),
        ]);
        let mut session = mock_session(transport);

        let results = report_channel_status(&mut session).expect("report_channel_status failed");

        // CHL.A (defined + running), CHL.B (defined + inactive), CHL.ORPHAN (no def)
        assert_eq!(results.len(), 3);

        let chl_a = results.iter().find(|c| c.name == "CHL.A").unwrap();
        assert!(chl_a.defined);
        assert_eq!(chl_a.status, "RUNNING");
        assert_eq!(chl_a.channel_type, "SDR");

        let chl_b = results.iter().find(|c| c.name == "CHL.B").unwrap();
        assert!(chl_b.defined);
        assert_eq!(chl_b.status, "INACTIVE");

        let orphan = results.iter().find(|c| c.name == "CHL.ORPHAN").unwrap();
        assert!(!orphan.defined);
        assert_eq!(orphan.status, "STOPPED");
    }

    #[test]
    fn channel_status_empty_names_filtered() {
        let mut empty_def = HashMap::new();
        empty_def.insert("channel_name".into(), json!(""));
        empty_def.insert("channel_type".into(), json!("SDR"));
        empty_def.insert("connection_name".into(), json!(""));

        let mut empty_status = HashMap::new();
        empty_status.insert("channel_name".into(), json!(""));
        empty_status.insert("status".into(), json!("RUNNING"));

        let transport = MockTransport::new(vec![
            // display_channel: one valid, one empty-name
            success_response(vec![
                channel_def_response("CHL.A", "SDR", "host(1414)"),
                empty_def,
            ]),
            // display_chstatus: one valid, one empty-name
            success_response(vec![chstatus_response("CHL.A", "RUNNING"), empty_status]),
        ]);
        let mut session = mock_session(transport);

        let results = report_channel_status(&mut session).expect("should succeed");
        // Only CHL.A should appear — empty names are filtered
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "CHL.A");
    }

    #[test]
    fn channel_status_no_live_status() {
        let transport = MockTransport::new(vec![
            // display_channel
            success_response(vec![channel_def_response("CHL.A", "SVRCONN", "")]),
            // display_chstatus returns error (no channels running)
            crate::test_helpers::error_response(2, 3065),
        ]);
        let mut session = mock_session(transport);

        let results = report_channel_status(&mut session).expect("report_channel_status failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, "INACTIVE");
    }

    // inspect_dlq -----------------------------------------------------------

    #[test]
    fn dlq_configured_empty() {
        let transport = MockTransport::new(vec![
            // display_qmgr
            success_response(vec![qmgr_with_dlq("DEAD.LETTER.Q")]),
            // display_queue (DLQ details)
            success_response(vec![local_queue_response("DEAD.LETTER.Q", 0, 5000, 0, 0)]),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(report.configured);
        assert_eq!(report.dlq_name, "DEAD.LETTER.Q");
        assert_eq!(report.current_depth, 0);
        assert_eq!(report.suggestion, "DLQ is empty. No action needed.");
    }

    #[test]
    fn dlq_not_configured() {
        let transport = MockTransport::new(vec![
            // display_qmgr with no DLQ
            success_response(vec![{
                let mut m = HashMap::new();
                m.insert("dead_letter_queue_name".into(), json!(""));
                m
            }]),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(!report.configured);
        assert!(report.dlq_name.is_empty());
        assert!(
            report
                .suggestion
                .contains("No dead letter queue configured")
        );
    }

    #[test]
    fn dlq_has_messages() {
        let transport = MockTransport::new(vec![
            success_response(vec![qmgr_with_dlq("DLQ")]),
            success_response(vec![local_queue_response("DLQ", 10, 5000, 0, 0)]),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(report.configured);
        assert_eq!(report.current_depth, 10);
        assert!(report.suggestion.contains("DLQ has messages"));
    }

    #[test]
    fn dlq_near_capacity() {
        let transport = MockTransport::new(vec![
            success_response(vec![qmgr_with_dlq("DLQ")]),
            success_response(vec![local_queue_response("DLQ", 950, 1000, 0, 0)]),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(report.depth_pct >= 90.0);
        assert!(report.suggestion.contains("near capacity"));
    }

    #[test]
    fn dlq_queue_does_not_exist() {
        let transport = MockTransport::new(vec![
            success_response(vec![qmgr_with_dlq("MISSING.DLQ")]),
            // display_queue returns empty
            empty_success_response(),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert!(report.configured);
        assert!(report.suggestion.contains("does not exist"));
    }

    #[test]
    fn dlq_zero_max_depth() {
        let transport = MockTransport::new(vec![
            success_response(vec![qmgr_with_dlq("DLQ")]),
            success_response(vec![local_queue_response("DLQ", 0, 0, 0, 0)]),
        ]);
        let mut session = mock_session(transport);

        let report = inspect_dlq(&mut session).expect("inspect_dlq failed");
        assert_eq!(report.depth_pct, 0.0);
    }

    // report_queue_handles --------------------------------------------------

    #[test]
    fn queue_handles_with_results() {
        let transport = MockTransport::new(vec![success_response(vec![queue_handle_response(
            "Q1", "ACTIVE", "CONN1", "INPUT",
        )])]);
        let mut session = mock_session(transport);

        let results = report_queue_handles(&mut session).expect("report_queue_handles failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].queue_name, "Q1");
        assert_eq!(results[0].handle_state, "ACTIVE");
    }

    #[test]
    fn queue_handles_empty() {
        let transport = MockTransport::new(vec![
            // display_qstatus returns error (no handles)
            crate::test_helpers::error_response(2, 3065),
        ]);
        let mut session = mock_session(transport);

        let results = report_queue_handles(&mut session).expect("report_queue_handles failed");
        assert!(results.is_empty());
    }

    // report_connection_handles ---------------------------------------------

    #[test]
    fn connection_handles_with_results() {
        let transport = MockTransport::new(vec![success_response(vec![conn_handle_response(
            "CONN1", "Q1", "ACTIVE", "QUEUE",
        )])]);
        let mut session = mock_session(transport);

        let results =
            report_connection_handles(&mut session).expect("report_connection_handles failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].connection_id, "CONN1");
        assert_eq!(results[0].object_name, "Q1");
    }

    #[test]
    fn connection_handles_empty() {
        let transport = MockTransport::new(vec![crate::test_helpers::error_response(2, 3065)]);
        let mut session = mock_session(transport);

        let results =
            report_connection_handles(&mut session).expect("report_connection_handles failed");
        assert!(results.is_empty());
    }

    // provision + teardown --------------------------------------------------

    #[test]
    fn provision_succeeds() {
        // provision calls: 6 defines (queues) + 4 defines (channels) + 2 verify queries
        let mut responses = Vec::new();

        // 10 define calls — all succeed
        for _ in 0..10 {
            responses.push(empty_success_response());
        }

        // verification: display_queue on QM1
        responses.push(success_response(vec![
            local_queue_response("PROV.QM1.LOCAL", 0, 5000, 0, 0),
            local_queue_response("PROV.QM1.TO.QM2.XMITQ", 0, 5000, 0, 0),
            local_queue_response("PROV.REMOTE.TO.QM2", 0, 5000, 0, 0),
        ]));
        // verification: display_queue on QM2
        responses.push(success_response(vec![
            local_queue_response("PROV.QM2.LOCAL", 0, 5000, 0, 0),
            local_queue_response("PROV.QM2.TO.QM1.XMITQ", 0, 5000, 0, 0),
            local_queue_response("PROV.REMOTE.TO.QM1", 0, 5000, 0, 0),
        ]));

        // Split responses: odd-indexed go to QM2, even-indexed to QM1.
        // Actually, provision calls define_queues then define_channels with
        // alternating QM1/QM2 calls. Let me build two separate transports.
        // The order is:
        //   define_queues: qm1, qm2, qm1, qm2, qm1, qm2 (6 calls)
        //   define_channels: qm1, qm2, qm2, qm1 (4 calls)
        //   verify: qm1, qm2 (2 calls)
        // QM1 gets calls at indices: 0, 2, 4, 6, 9, 10 (6 calls)
        // QM2 gets calls at indices: 1, 3, 5, 7, 8, 11 (6 calls)

        let qm1_responses = vec![
            empty_success_response(), // define qm1 local
            empty_success_response(), // define qm1 xmitq
            empty_success_response(), // define qm1 remote
            empty_success_response(), // define qm1 channel sdr
            empty_success_response(), // define qm1 channel rcvr
            // verify qm1
            success_response(vec![
                local_queue_response("PROV.QM1.LOCAL", 0, 5000, 0, 0),
                local_queue_response("PROV.QM1.TO.QM2.XMITQ", 0, 5000, 0, 0),
                local_queue_response("PROV.REMOTE.TO.QM2", 0, 5000, 0, 0),
            ]),
        ];

        let qm2_responses = vec![
            empty_success_response(), // define qm2 local
            empty_success_response(), // define qm2 xmitq
            empty_success_response(), // define qm2 remote
            empty_success_response(), // define qm2 channel rcvr
            empty_success_response(), // define qm2 channel sdr
            // verify qm2
            success_response(vec![
                local_queue_response("PROV.QM2.LOCAL", 0, 5000, 0, 0),
                local_queue_response("PROV.QM2.TO.QM1.XMITQ", 0, 5000, 0, 0),
                local_queue_response("PROV.REMOTE.TO.QM1", 0, 5000, 0, 0),
            ]),
        ];

        let transport1 = MockTransport::new(qm1_responses);
        let transport2 = MockTransport::new(qm2_responses);
        let mut qm1 = mock_session(transport1);
        let mut qm2 = mock_session(transport2);

        let result = provision(&mut qm1, &mut qm2).expect("provision failed");
        assert_eq!(result.objects_created.len(), 10);
        assert!(result.failures.is_empty());
        assert!(result.verified);
    }

    #[test]
    fn provision_with_failures() {
        // All calls fail
        let qm1_responses = vec![
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            // verification still runs
            empty_success_response(),
        ];
        let qm2_responses = vec![
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            crate::test_helpers::error_response(2, 3000),
            empty_success_response(),
        ];

        let mut qm1 = mock_session(MockTransport::new(qm1_responses));
        let mut qm2 = mock_session(MockTransport::new(qm2_responses));

        let result = provision(&mut qm1, &mut qm2).expect("provision failed");
        assert!(result.objects_created.is_empty());
        assert_eq!(result.failures.len(), 10);
        assert!(!result.verified);
    }

    #[test]
    fn provision_verification_transport_error() {
        // All defines succeed, but verification calls exhaust the transport
        let mut qm1_responses = Vec::new();
        for _ in 0..5 {
            qm1_responses.push(empty_success_response());
        }
        // QM1 verification returns error (transport exhausted)

        let mut qm2_responses = Vec::new();
        for _ in 0..5 {
            qm2_responses.push(empty_success_response());
        }
        // QM2 verification: no response queued — will error

        let mut qm1 = mock_session(MockTransport::new(qm1_responses));
        let mut qm2 = mock_session(MockTransport::new(qm2_responses));

        let result = provision(&mut qm1, &mut qm2).expect("provision failed");
        assert!(!result.verified);
    }

    #[test]
    fn teardown_succeeds() {
        // teardown calls: 2 channels + 6 queues per session = 8 per QM, 16 total
        let qm1_responses: Vec<_> = (0..8).map(|_| empty_success_response()).collect();
        let qm2_responses: Vec<_> = (0..8).map(|_| empty_success_response()).collect();

        let mut qm1 = mock_session(MockTransport::new(qm1_responses));
        let mut qm2 = mock_session(MockTransport::new(qm2_responses));

        let failures = teardown(&mut qm1, &mut qm2).expect("teardown failed");
        assert!(failures.is_empty());
    }

    #[test]
    fn teardown_with_failures() {
        let qm1_responses: Vec<_> = (0..8)
            .map(|_| crate::test_helpers::error_response(2, 3000))
            .collect();
        let qm2_responses: Vec<_> = (0..8)
            .map(|_| crate::test_helpers::error_response(2, 3000))
            .collect();

        let mut qm1 = mock_session(MockTransport::new(qm1_responses));
        let mut qm2 = mock_session(MockTransport::new(qm2_responses));

        let failures = teardown(&mut qm1, &mut qm2).expect("teardown failed");
        assert_eq!(failures.len(), 16);
    }

    // Error propagation tests ------------------------------------------------

    #[test]
    fn monitor_queue_depths_transport_error() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = monitor_queue_depths(&mut session, 80.0);
        assert!(result.is_err());
    }

    #[test]
    fn report_channel_status_transport_error() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = report_channel_status(&mut session);
        assert!(result.is_err());
    }

    #[test]
    fn inspect_dlq_transport_error() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = inspect_dlq(&mut session);
        assert!(result.is_err());
    }

    #[test]
    fn inspect_dlq_display_queue_transport_error() {
        let transport = MockTransport::new(vec![
            success_response(vec![qmgr_with_dlq("DLQ")]),
            // display_queue fails
        ]);
        let mut session = mock_session(transport);
        let result = inspect_dlq(&mut session);
        assert!(result.is_err());
    }

    // get_str / get_i64 helpers ---------------------------------------------

    #[test]
    fn get_str_missing_key() {
        let m: HashMap<String, Value> = HashMap::new();
        assert_eq!(super::get_str(&m, "missing"), "");
    }

    #[test]
    fn get_str_non_string() {
        let mut m = HashMap::new();
        m.insert("num".into(), json!(42));
        assert_eq!(super::get_str(&m, "num"), "");
    }

    #[test]
    fn get_str_trimmed() {
        let mut m = HashMap::new();
        m.insert("val".into(), json!("  hello  "));
        assert_eq!(super::get_str(&m, "val"), "hello");
    }

    #[test]
    fn get_i64_from_number() {
        let mut m = HashMap::new();
        m.insert("val".into(), json!(42));
        assert_eq!(super::get_i64(&m, "val"), 42);
    }

    #[test]
    fn get_i64_from_string() {
        let mut m = HashMap::new();
        m.insert("val".into(), json!("  99  "));
        assert_eq!(super::get_i64(&m, "val"), 99);
    }

    #[test]
    fn get_i64_from_invalid_string() {
        let mut m = HashMap::new();
        m.insert("val".into(), json!("abc"));
        assert_eq!(super::get_i64(&m, "val"), 0);
    }

    #[test]
    fn get_i64_missing_key() {
        let m: HashMap<String, Value> = HashMap::new();
        assert_eq!(super::get_i64(&m, "missing"), 0);
    }

    #[test]
    fn get_i64_null_value() {
        let mut m = HashMap::new();
        m.insert("val".into(), Value::Null);
        assert_eq!(super::get_i64(&m, "val"), 0);
    }

    #[test]
    fn get_i64_float_number() {
        let mut m = HashMap::new();
        m.insert("val".into(), json!(3.14));
        // as_i64() returns None for floats
        assert_eq!(super::get_i64(&m, "val"), 0);
    }
}
