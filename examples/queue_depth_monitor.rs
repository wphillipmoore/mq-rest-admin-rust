//! Queue depth monitor.
//!
//! Displays local queues with their current depth, flags queues that
//! are approaching capacity, and sorts by depth percentage descending.
//!
//! ```text
//! cargo run --example queue_depth_monitor
//! ```
//!
//! Set `DEPTH_THRESHOLD_PCT` to change the warning threshold (default 80).

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

fn get_i64(map: &HashMap<String, Value>, key: &str) -> i64 {
    match map.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

struct QueueDepthInfo {
    name: String,
    current_depth: i64,
    max_depth: i64,
    depth_pct: f64,
    open_input: i64,
    open_output: i64,
    warning: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9443/ibmmq/rest/v2".into());
    let qmgr_name = env::var("MQ_QMGR_NAME").unwrap_or_else(|_| "QM1".into());
    let username = env::var("MQ_ADMIN_USER").unwrap_or_else(|_| "mqadmin".into());
    let password = env::var("MQ_ADMIN_PASSWORD").unwrap_or_else(|_| "mqadmin".into());
    let threshold_pct: f64 = env::var("DEPTH_THRESHOLD_PCT")
        .unwrap_or_else(|_| "80".into())
        .parse()
        .unwrap_or(80.0);

    let mut session = MqRestSession::builder(
        &rest_base_url,
        &qmgr_name,
        Credentials::Ltpa { username, password },
    )
    .verify_tls(false)
    .build()?;

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

    println!(
        "\n{:<40} {:>8} {:>8} {:>6} {:>4} {:>4} Status",
        "Queue", "Depth", "Max", "%", "In", "Out"
    );
    println!("{}", "-".repeat(90));

    for info in &results {
        let status = if info.warning { "WARNING" } else { "OK" };
        println!(
            "{:<40} {:>8} {:>8} {:>5.1}% {:>4} {:>4} {status}",
            info.name,
            info.current_depth,
            info.max_depth,
            info.depth_pct,
            info.open_input,
            info.open_output
        );
    }

    let warning_count = results.iter().filter(|q| q.warning).count();
    println!(
        "\nTotal queues: {}, warnings: {warning_count}",
        results.len()
    );

    Ok(())
}
