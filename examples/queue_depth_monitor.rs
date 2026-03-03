//! Queue depth monitor.
//!
//! Displays local queues with their current depth, flags queues that
//! are approaching capacity, and sorts by depth percentage descending.
//!
//! ```text
//! cargo run --features examples --example queue_depth_monitor
//! ```
//!
//! Set `DEPTH_THRESHOLD_PCT` to change the warning threshold (default 80).

use std::env;

use mq_rest_admin::{Credentials, MqRestSession, examples};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9483/ibmmq/rest/v2".into());
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

    let results = examples::monitor_queue_depths(&mut session, threshold_pct)?;

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
