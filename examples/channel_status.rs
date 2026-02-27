//! Channel status report.
//!
//! Displays channel definitions alongside live channel status, identifies
//! channels that are defined but not running, and shows connection details.
//!
//! ```text
//! cargo run --example channel_status
//! ```

use std::collections::HashMap;
use std::env;

use mq_rest_admin::{Credentials, MqRestSession};
use serde_json::Value;

struct ChannelInfo {
    name: String,
    channel_type: String,
    connection_name: String,
    defined: bool,
    status: String,
}

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

    // Collect channel definitions
    let channels = session.display_channel(Some("*"), None, None, None)?;
    let mut definitions: HashMap<String, HashMap<String, Value>> = HashMap::new();
    for channel in channels {
        let cname = get_str(&channel, "channel_name");
        if !cname.is_empty() {
            definitions.insert(cname, channel);
        }
    }

    // Collect live status
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

    // Merge definitions and status
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

    // Channels with status but no definition
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

    // Print report
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

    let inactive: Vec<&ChannelInfo> = results
        .iter()
        .filter(|c| c.defined && c.status == "INACTIVE")
        .collect();
    if !inactive.is_empty() {
        let names: Vec<&str> = inactive.iter().map(|c| c.name.as_str()).collect();
        println!("\nDefined but inactive: {}", names.join(", "));
    }

    Ok(())
}
