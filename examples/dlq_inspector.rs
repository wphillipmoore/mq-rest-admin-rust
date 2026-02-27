//! Dead letter queue inspector.
//!
//! Checks the dead letter queue configuration for a queue manager,
//! reports its depth and capacity, and suggests actions when messages
//! are present.
//!
//! ```text
//! cargo run --example dlq_inspector
//! ```

use std::collections::HashMap;
use std::env;

use mq_rest_admin::{Credentials, MqRestSession};
use serde_json::Value;

const CRITICAL_DEPTH_PCT: f64 = 90.0;

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

    let qmgr = session.display_qmgr(None, None)?;

    let dlq_name = qmgr
        .as_ref()
        .map(|q| get_str(q, "dead_letter_queue_name"))
        .unwrap_or_default();

    println!("\n=== Dead Letter Queue: {} ===", session.qmgr_name());

    if dlq_name.is_empty() {
        println!("  Configured: false");
        println!("  DLQ name:   (none)");
        println!(
            "  Suggestion: No dead letter queue configured. Define one with ALTER QMGR DEADQ."
        );
        return Ok(());
    }

    println!("  Configured: true");
    println!("  DLQ name:   {dlq_name}");

    let queues = session.display_queue(Some(&dlq_name), None, None, None)?;
    if queues.is_empty() {
        println!("  Suggestion: DLQ '{dlq_name}' is configured but the queue does not exist.");
        return Ok(());
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

    println!("  Depth:      {current_depth} / {max_depth} ({depth_pct:.1}%)");
    println!("  Input:      {open_input}");
    println!("  Output:     {open_output}");

    let suggestion = if current_depth == 0 {
        "DLQ is empty. No action needed."
    } else if depth_pct >= CRITICAL_DEPTH_PCT {
        "DLQ is near capacity. Investigate and clear undeliverable messages urgently."
    } else if current_depth > 0 {
        "DLQ has messages. Investigate undeliverable messages."
    } else {
        "DLQ is healthy."
    };
    println!("  Suggestion: {suggestion}");

    Ok(())
}
