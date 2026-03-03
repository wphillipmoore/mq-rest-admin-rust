//! Queue status and connection handle report.
//!
//! Demonstrates `DISPLAY QSTATUS TYPE(HANDLE)` and `DISPLAY CONN
//! TYPE(HANDLE)` queries, showing how `mq-rest-admin` transparently
//! flattens the nested `objects` response structure into uniform flat
//! `HashMap`s.
//!
//! ```text
//! cargo run --features examples --example queue_status
//! ```

use std::env;

use mq_rest_admin::{Credentials, MqRestSession, examples};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9483/ibmmq/rest/v2".into());
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
    let queue_handles = examples::report_queue_handles(&mut session)?;

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
                entry.queue_name, entry.handle_state, entry.connection_id, entry.open_options,
            );
        }
    }

    // Connection handles with TYPE(HANDLE)
    let conn_handles = examples::report_connection_handles(&mut session)?;

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
                entry.connection_id, entry.object_name, entry.handle_state, entry.object_type,
            );
        }
    }

    Ok(())
}
