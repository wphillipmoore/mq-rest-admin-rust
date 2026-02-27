//! Queue status and connection handle report.
//!
//! Demonstrates `DISPLAY QSTATUS TYPE(HANDLE)` and `DISPLAY CONN
//! TYPE(HANDLE)` queries, showing how `mq-rest-admin` transparently
//! flattens the nested `objects` response structure into uniform flat
//! `HashMap`s.
//!
//! ```text
//! cargo run --example queue_status
//! ```

use std::collections::HashMap;
use std::env;

use mq_rest_admin::{Credentials, MqRestSession};
use serde_json::Value;

fn get_str(map: &HashMap<String, Value>, key: &str) -> String {
    map.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9443/ibmmq/rest/v2".into());
    let qmgr_name = env::var("MQ_QMGR_NAME").unwrap_or_else(|_| "QM1".into());
    let username = env::var("MQ_ADMIN_USER").unwrap_or_else(|_| "mqadmin".into());
    let password = env::var("MQ_ADMIN_PASSWORD").unwrap_or_else(|_| "mqadmin".into());

    let mut session = MqRestSession::builder(
        &rest_base_url,
        &qmgr_name,
        Credentials::Ltpa { username, password },
    )
    .verify_tls(false)
    .build()?;

    // Queue status with TYPE(HANDLE)
    let mut qstatus_params: HashMap<String, Value> = HashMap::new();
    qstatus_params.insert("type".into(), Value::String("HANDLE".into()));

    let queue_handles = session
        .display_qstatus(Some("*"), Some(&qstatus_params), None, None)
        .unwrap_or_default();

    println!(
        "\n{:<30} {:<15} {:<30} Open Options",
        "Queue", "Handle State", "Connection ID"
    );
    println!("{}", "-".repeat(90));

    if queue_handles.is_empty() {
        println!("  (no active queue handles)");
    } else {
        for entry in &queue_handles {
            println!(
                "{:<30} {:<15} {:<30} {}",
                get_str(entry, "queue_name"),
                get_str(entry, "handle_state"),
                get_str(entry, "connection_id"),
                get_str(entry, "open_options"),
            );
        }
    }

    // Connection handles with TYPE(HANDLE)
    let mut conn_params: HashMap<String, Value> = HashMap::new();
    conn_params.insert(
        "connection_info_type".into(),
        Value::String("HANDLE".into()),
    );

    let conn_handles = session
        .display_conn(Some("*"), Some(&conn_params), None, None)
        .unwrap_or_default();

    println!(
        "\n{:<30} {:<30} {:<15} Object Type",
        "Connection ID", "Object Name", "Handle State"
    );
    println!("{}", "-".repeat(90));

    if conn_handles.is_empty() {
        println!("  (no active connection handles)");
    } else {
        for entry in &conn_handles {
            println!(
                "{:<30} {:<30} {:<15} {}",
                get_str(entry, "connection_id"),
                get_str(entry, "object_name"),
                get_str(entry, "handle_state"),
                get_str(entry, "object_type"),
            );
        }
    }

    Ok(())
}
