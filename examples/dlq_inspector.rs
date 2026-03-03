//! Dead letter queue inspector.
//!
//! Checks the dead letter queue configuration for a queue manager,
//! reports its depth and capacity, and suggests actions when messages
//! are present.
//!
//! ```text
//! cargo run --features examples --example dlq_inspector
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

    let report = examples::inspect_dlq(&mut session)?;

    println!("\n=== Dead Letter Queue: {} ===", session.qmgr_name());
    println!("  Configured: {}", report.configured);

    if report.dlq_name.is_empty() {
        println!("  DLQ name:   (none)");
    } else {
        println!("  DLQ name:   {}", report.dlq_name);
        println!(
            "  Depth:      {} / {} ({:.1}%)",
            report.current_depth, report.max_depth, report.depth_pct
        );
        println!("  Input:      {}", report.open_input);
        println!("  Output:     {}", report.open_output);
    }

    println!("  Suggestion: {}", report.suggestion);

    Ok(())
}
