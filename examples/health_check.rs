//! Queue manager health check.
//!
//! Connects to one or more queue managers and checks QMGR status,
//! command server availability, and listener state. Produces a
//! pass/fail summary for each queue manager.
//!
//! ```text
//! cargo run --features examples --example health_check
//! ```
//!
//! Set `MQ_REST_BASE_URL_QM2` to also check QM2.

use std::env;

use mq_rest_admin::{Credentials, MqRestSession, examples};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_base_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9483/ibmmq/rest/v2".into());
    let qmgr_name = env::var("MQ_QMGR_NAME").unwrap_or_else(|_| "QM1".into());
    let username = env::var("MQ_ADMIN_USER").unwrap_or_else(|_| "mqadmin".into());
    let password = env::var("MQ_ADMIN_PASSWORD").unwrap_or_else(|_| "mqadmin".into());

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
        let result = examples::check_health(session)?;
        let verdict = if result.passed { "PASS" } else { "FAIL" };

        println!("\n=== {label}: {verdict} ===");
        println!("  Reachable:      {}", result.reachable);
        println!("  Status:         {}", result.status);
        println!("  Command server: {}", result.command_server);
        println!("  Listeners:      {}", result.listeners.len());
        for listener in &result.listeners {
            println!("    {}: {}", listener.name, listener.start_mode);
        }
    }

    Ok(())
}
