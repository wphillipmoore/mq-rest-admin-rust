//! Compile-time loading of the bundled mapping data JSON.

use std::sync::LazyLock;

use serde_json::Value;

/// The bundled mapping data parsed from `mapping-data.json`.
pub static MAPPING_DATA: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../mapping-data.json"))
        .expect("bundled mapping-data.json must be valid JSON")
});
