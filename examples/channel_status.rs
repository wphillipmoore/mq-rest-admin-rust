//! Channel status report.
//!
//! Displays channel definitions alongside live channel status, identifies
//! channels that are defined but not running, and shows connection details.
//!
//! ```text
//! cargo run --features examples --example channel_status
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

    let results = examples::report_channel_status(&mut session)?;

    println!(
        "\n{:<30} {:<12} {:<25} {:<8} Status",
        "Channel", "Type", "Connection", "Defined"
    );
    println!("{}", "-".repeat(90));

    for info in &results {
        let defined = if info.defined { "Yes" } else { "No" };
        println!(
            "{:<30} {:<12} {:<25} {:<8} {}",
            info.name, info.channel_type, info.connection_name, defined, info.status
        );
    }

    let inactive: Vec<_> = results
        .iter()
        .filter(|c| c.defined && c.status == "INACTIVE")
        .collect();
    if !inactive.is_empty() {
        let names: Vec<&str> = inactive.iter().map(|c| c.name.as_str()).collect();
        println!("\nDefined but inactive: {}", names.join(", "));
    }

    Ok(())
}
