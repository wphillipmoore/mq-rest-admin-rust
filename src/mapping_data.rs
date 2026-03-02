//! Compile-time loading of the bundled mapping data JSON.

use std::sync::LazyLock;

use serde_json::Value;

/// The bundled mapping data parsed from `mapping-data.json`.
pub static MAPPING_DATA: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../mapping-data.json"))
        .expect("bundled mapping-data.json must be valid JSON")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_data_is_valid_json_object() {
        assert!(MAPPING_DATA.is_object());
    }

    #[test]
    fn mapping_data_has_commands_and_qualifiers() {
        assert!(MAPPING_DATA.get("commands").is_some());
        assert!(MAPPING_DATA.get("qualifiers").is_some());
    }
}
