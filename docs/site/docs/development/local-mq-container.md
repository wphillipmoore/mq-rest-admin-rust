# Local MQ Container

--8<-- "development/local-mq-container.md"

## Rust-specific notes

### Integration tests

Integration tests are opt-in and require running MQ containers:

```bash
MQ_REST_ADMIN_RUN_INTEGRATION=1 cargo test --features integration
```

When enabled, the test session connects to the local MQ containers and
runs DISPLAY checks plus define/alter/delete lifecycles against both
queue managers.

### Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `MQ_REST_BASE_URL` | `https://localhost:9483/ibmmq/rest/v2` | QM1 REST API base URL |
| `MQ_REST_BASE_URL_QM2` | `https://localhost:9484/ibmmq/rest/v2` | QM2 REST API base URL |
| `MQ_ADMIN_USER` | `mqadmin` | Admin username |
| `MQ_ADMIN_PASSWORD` | `mqadmin` | Admin password |
| `MQ_IMAGE` | `icr.io/ibm-messaging/mq:latest` | Container image |

### Gateway routing with mq-rest-admin

```rust
use mq_rest_admin::{MqRestSession, Credentials};

// Route commands to QM2 through QM1
let session = MqRestSession::builder()
    .rest_base_url("https://localhost:9483/ibmmq/rest/v2")
    .qmgr_name("QM2")
    .credentials(Credentials::Basic {
        username: "mqadmin".into(),
        password: "mqadmin".into(),
    })
    .gateway_qmgr("QM1")
    .verify_tls(false)
    .build()?;

if let Some(qmgr) = session.display_qmgr(None, None)? {
    println!("{:?}", qmgr);  // QM2's attributes, routed through QM1
}
```
