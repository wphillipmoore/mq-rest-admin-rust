//! Environment provisioner.
//!
//! Defines a complete set of queues, channels, and remote queue
//! definitions across two queue managers, then verifies connectivity.
//! Includes teardown to remove all provisioned objects.
//!
//! ```text
//! cargo run --features examples --example provision_environment
//! ```
//!
//! Requires both QM1 and QM2 to be running. Set `MQ_REST_BASE_URL_QM2`
//! to the QM2 REST endpoint (default: `https://localhost:9444/ibmmq/rest/v2`).

use std::env;

use mq_rest_admin::{Credentials, MqRestSession, examples};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = env::var("MQ_ADMIN_USER").unwrap_or_else(|_| "mqadmin".into());
    let password = env::var("MQ_ADMIN_PASSWORD").unwrap_or_else(|_| "mqadmin".into());

    let qm1_url = env::var("MQ_REST_BASE_URL")
        .unwrap_or_else(|_| "https://localhost:9443/ibmmq/rest/v2".into());
    let qm2_url = env::var("MQ_REST_BASE_URL_QM2")
        .unwrap_or_else(|_| "https://localhost:9444/ibmmq/rest/v2".into());

    let mut qm1 = MqRestSession::builder(
        &qm1_url,
        "QM1",
        Credentials::Ltpa {
            username: username.clone(),
            password: password.clone(),
        },
    )
    .verify_tls(false)
    .build()?;

    let mut qm2 = MqRestSession::builder(&qm2_url, "QM2", Credentials::Ltpa { username, password })
        .verify_tls(false)
        .build()?;

    println!("\n=== Provisioning environment ===");
    let result = examples::provision(&mut qm1, &mut qm2)?;

    println!("\nCreated: {}", result.objects_created.len());
    for obj in &result.objects_created {
        println!("  + {obj}");
    }
    if !result.failures.is_empty() {
        println!("\nFailed: {}", result.failures.len());
        for obj in &result.failures {
            println!("  ! {obj}");
        }
    }
    println!("\nVerified: {}", result.verified);

    println!("\n=== Tearing down ===");
    let teardown_failures = examples::teardown(&mut qm1, &mut qm2)?;
    if teardown_failures.is_empty() {
        println!("Teardown complete.");
    } else {
        println!("Teardown failures: {teardown_failures:?}");
    }

    Ok(())
}
