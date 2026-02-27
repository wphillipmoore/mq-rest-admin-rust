//! Queue manager health check.
//!
//! Connects to one or more queue managers and checks QMGR status,
//! command server availability, and listener state. Produces a
//! pass/fail summary for each queue manager.
//!
//! ```text
//! cargo run --example health_check
//! ```
//!
//! Set `MQ_REST_BASE_URL_QM2` to also check QM2.

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

fn check_health(session: &mut MqRestSession) -> (bool, String, String, Vec<(String, String)>) {
    let mut status = "UNKNOWN".to_string();
    let mut command_server = "UNKNOWN".to_string();
    let mut listeners: Vec<(String, String)> = Vec::new();

    let Ok(_qmgr) = session.display_qmgr(None, None) else {
        return (false, status, command_server, listeners);
    };

    if let Ok(Some(qs)) = session.display_qmstatus(None, None) {
        status = get_str(&qs, "ha_status");
    }

    if let Ok(Some(cs)) = session.display_cmdserv(None, None) {
        command_server = get_str(&cs, "status");
    }

    if let Ok(lst) = session.display_listener(Some("*"), None, None, None) {
        for listener in lst {
            let lname = get_str(&listener, "listener_name");
            let lstatus = get_str(&listener, "start_mode");
            listeners.push((lname, lstatus));
        }
    }

    let passed = status != "UNKNOWN";
    (passed, status, command_server, listeners)
}

fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} environment variable is required"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = require_env("MQ_REST_BASE_URL");
    let qmgr_name = require_env("MQ_QMGR_NAME");
    let username = require_env("MQ_ADMIN_USER");
    let password = require_env("MQ_ADMIN_PASSWORD");

    let mut sessions: Vec<(String, MqRestSession)> = Vec::new();

    sessions.push((
        qmgr_name.clone(),
        MqRestSession::builder(
            &rest_base_url,
            &qmgr_name,
            Credentials::Ltpa {
                username: username.clone(),
                password: password.clone(),
            },
        )
        .verify_tls(false)
        .build()?,
    ));

    if let Ok(qm2_url) = env::var("MQ_REST_BASE_URL_QM2") {
        sessions.push((
            "QM2".into(),
            MqRestSession::builder(&qm2_url, "QM2", Credentials::Ltpa { username, password })
                .verify_tls(false)
                .build()?,
        ));
    }

    for (label, session) in &mut sessions {
        let (passed, status, command_server, listeners) = check_health(session);
        let verdict = if passed { "PASS" } else { "FAIL" };

        println!("\n=== {label}: {verdict} ===");
        println!("  Reachable:      {}", passed || status != "UNKNOWN");
        println!("  Status:         {status}");
        println!("  Command server: {command_server}");
        println!("  Listeners:      {}", listeners.len());
        for (lname, lstatus) in &listeners {
            println!("    {lname}: {lstatus}");
        }
    }

    Ok(())
}
