//! Validation and merging of mapping overrides.

use serde_json::Value;

/// Mode for applying mapping overrides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingOverrideMode {
    /// Sparse merge — override entries are layered on top of the built-in data.
    Merge,
    /// Complete replacement — override data replaces the built-in data entirely.
    Replace,
}

const VALID_TOP_LEVEL_KEYS: &[&str] = &["commands", "qualifiers"];

const VALID_QUALIFIER_SUB_KEYS: &[&str] = &[
    "request_key_map",
    "request_key_value_map",
    "request_value_map",
    "response_key_map",
    "response_value_map",
];

/// Validate the structure of a mapping overrides value.
///
/// # Errors
///
/// Returns `Err` with a descriptive message for type violations or invalid keys.
pub fn validate_mapping_overrides(overrides: &Value) -> Result<(), String> {
    let obj = overrides
        .as_object()
        .ok_or("mapping_overrides must be a JSON object")?;
    for key in obj.keys() {
        if !VALID_TOP_LEVEL_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "Invalid top-level key in mapping_overrides: {key:?} (expected subset of {VALID_TOP_LEVEL_KEYS:?})"
            ));
        }
    }
    validate_commands_section(obj.get("commands"))?;
    validate_qualifiers_section(obj.get("qualifiers"))?;
    Ok(())
}

fn validate_commands_section(commands: Option<&Value>) -> Result<(), String> {
    let Some(commands) = commands else {
        return Ok(());
    };
    let obj = commands
        .as_object()
        .ok_or("mapping_overrides['commands'] must be an object")?;
    for (key, entry) in obj {
        if !entry.is_object() {
            return Err(format!(
                "mapping_overrides['commands'][{key:?}] must be an object"
            ));
        }
    }
    Ok(())
}

fn validate_qualifiers_section(qualifiers: Option<&Value>) -> Result<(), String> {
    let Some(qualifiers) = qualifiers else {
        return Ok(());
    };
    let obj = qualifiers
        .as_object()
        .ok_or("mapping_overrides['qualifiers'] must be an object")?;
    for (key, entry) in obj {
        let entry_obj = entry
            .as_object()
            .ok_or_else(|| format!("mapping_overrides['qualifiers'][{key:?}] must be an object"))?;
        validate_qualifier_entry(key, entry_obj)?;
    }
    Ok(())
}

fn validate_qualifier_entry(
    qualifier_key: &str,
    entry: &serde_json::Map<String, Value>,
) -> Result<(), String> {
    for sub_key in entry.keys() {
        if !VALID_QUALIFIER_SUB_KEYS.contains(&sub_key.as_str()) {
            return Err(format!(
                "Invalid sub-key {sub_key:?} in mapping_overrides['qualifiers'][{qualifier_key:?}] \
                 (expected subset of {VALID_QUALIFIER_SUB_KEYS:?})"
            ));
        }
        if !entry[sub_key].is_object() {
            return Err(format!(
                "mapping_overrides['qualifiers'][{qualifier_key:?}][{sub_key:?}] must be an object"
            ));
        }
    }
    Ok(())
}

/// Deep-copy `base` and merge `overrides` into it.
#[must_use]
pub fn merge_mapping_data(base: &Value, overrides: &Value) -> Value {
    let mut merged = base.clone();
    merge_commands(&mut merged, overrides.get("commands"));
    merge_qualifiers(&mut merged, overrides.get("qualifiers"));
    merged
}

fn merge_commands(merged: &mut Value, override_commands: Option<&Value>) {
    let Some(override_obj) = override_commands.and_then(Value::as_object) else {
        return;
    };
    let base_commands = merged
        .as_object_mut()
        .unwrap()
        .entry("commands")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));
    let base_obj = base_commands.as_object_mut().unwrap();
    for (key, entry) in override_obj {
        if let Some(entry_obj) = entry.as_object() {
            if let Some(existing) = base_obj.get_mut(key).and_then(Value::as_object_mut) {
                for (entry_key, entry_value) in entry_obj {
                    existing.insert(entry_key.clone(), entry_value.clone());
                }
            } else {
                base_obj.insert(key.clone(), entry.clone());
            }
        }
    }
}

fn merge_qualifiers(merged: &mut Value, override_qualifiers: Option<&Value>) {
    let Some(override_obj) = override_qualifiers.and_then(Value::as_object) else {
        return;
    };
    let base_qualifiers = merged
        .as_object_mut()
        .unwrap()
        .entry("qualifiers")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));
    let base_obj = base_qualifiers.as_object_mut().unwrap();
    for (key, entry) in override_obj {
        if let Some(entry_obj) = entry.as_object() {
            if let Some(existing) = base_obj.get_mut(key).and_then(Value::as_object_mut) {
                for (sub_key, sub_value) in entry_obj {
                    if let Some(sub_obj) = sub_value.as_object() {
                        if let Some(existing_sub) =
                            existing.get_mut(sub_key).and_then(Value::as_object_mut)
                        {
                            for (key, value) in sub_obj {
                                existing_sub.insert(key.clone(), value.clone());
                            }
                        } else {
                            existing.insert(sub_key.clone(), sub_value.clone());
                        }
                    }
                }
            } else {
                base_obj.insert(key.clone(), entry.clone());
            }
        }
    }
}

/// Validate that `overrides` covers all command and qualifier keys in `base`.
///
/// # Errors
///
/// Returns `Err` listing any command or qualifier keys present in `base` but
/// missing from `overrides`.
pub fn validate_mapping_overrides_complete(base: &Value, overrides: &Value) -> Result<(), String> {
    let mut missing_parts = Vec::new();

    if let Some(base_commands) = base.get("commands").and_then(Value::as_object) {
        let override_commands = overrides
            .get("commands")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut missing: Vec<&str> = base_commands
            .keys()
            .filter(|k| !override_commands.contains_key(k.as_str()))
            .map(String::as_str)
            .collect();
        missing.sort_unstable();
        for key in missing {
            missing_parts.push(format!("commands: {key}"));
        }
    }

    if let Some(base_qualifiers) = base.get("qualifiers").and_then(Value::as_object) {
        let override_qualifiers = overrides
            .get("qualifiers")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut missing: Vec<&str> = base_qualifiers
            .keys()
            .filter(|k| !override_qualifiers.contains_key(k.as_str()))
            .map(String::as_str)
            .collect();
        missing.sort_unstable();
        for key in missing {
            missing_parts.push(format!("qualifiers: {key}"));
        }
    }

    if !missing_parts.is_empty() {
        let detail: Vec<String> = missing_parts.iter().map(|e| format!("  {e}")).collect();
        return Err(format!(
            "mapping_overrides is incomplete for REPLACE mode. Missing entries:\n{}",
            detail.join("\n")
        ));
    }
    Ok(())
}

/// Return a deep copy of `overrides` as the complete mapping data.
#[must_use]
pub fn replace_mapping_data(overrides: &Value) -> Value {
    overrides.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ---- validate_mapping_overrides ----

    #[test]
    fn validate_valid_overrides() {
        let v = json!({"commands": {}, "qualifiers": {}});
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn validate_not_object() {
        let v = json!("string");
        assert!(validate_mapping_overrides(&v).is_err());
    }

    #[test]
    fn validate_invalid_top_key() {
        let v = json!({"bogus": {}});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("Invalid top-level key"));
    }

    #[test]
    fn validate_commands_not_object() {
        let v = json!({"commands": "bad"});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("commands"));
    }

    #[test]
    fn validate_commands_entry_not_object() {
        let v = json!({"commands": {"DISPLAY QUEUE": "bad"}});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("DISPLAY QUEUE"));
    }

    #[test]
    fn validate_qualifiers_not_object() {
        let v = json!({"qualifiers": "bad"});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("qualifiers"));
    }

    #[test]
    fn validate_qualifier_entry_not_object() {
        let v = json!({"qualifiers": {"queue": "bad"}});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("queue"));
    }

    #[test]
    fn validate_qualifier_invalid_sub_key() {
        let v = json!({"qualifiers": {"queue": {"bogus_key": {}}}});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("bogus_key"));
    }

    #[test]
    fn validate_qualifier_sub_value_not_object() {
        let v = json!({"qualifiers": {"queue": {"request_key_map": "bad"}}});
        let err = validate_mapping_overrides(&v).unwrap_err();
        assert!(err.contains("request_key_map"));
    }

    // ---- merge_mapping_data ----

    #[test]
    fn merge_new_command() {
        let base = json!({"commands": {"A": {"q": "x"}}, "qualifiers": {}});
        let overrides = json!({"commands": {"B": {"q": "y"}}});
        let merged = merge_mapping_data(&base, &overrides);
        assert!(merged["commands"]["A"].is_object());
        assert!(merged["commands"]["B"].is_object());
    }

    #[test]
    fn merge_existing_command() {
        let base = json!({"commands": {"A": {"old": "1"}}, "qualifiers": {}});
        let overrides = json!({"commands": {"A": {"new": "2"}}});
        let merged = merge_mapping_data(&base, &overrides);
        assert_eq!(merged["commands"]["A"]["old"], "1");
        assert_eq!(merged["commands"]["A"]["new"], "2");
    }

    #[test]
    fn merge_new_qualifier() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {"request_key_map": {"a": "A"}}}});
        let overrides = json!({"qualifiers": {"q2": {"request_key_map": {"b": "B"}}}});
        let merged = merge_mapping_data(&base, &overrides);
        assert!(merged["qualifiers"]["q1"].is_object());
        assert!(merged["qualifiers"]["q2"].is_object());
    }

    #[test]
    fn merge_existing_qualifier_nested() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {"request_key_map": {"a": "A"}}}});
        let overrides = json!({"qualifiers": {"q1": {"request_key_map": {"b": "B"}}}});
        let merged = merge_mapping_data(&base, &overrides);
        assert_eq!(merged["qualifiers"]["q1"]["request_key_map"]["a"], "A");
        assert_eq!(merged["qualifiers"]["q1"]["request_key_map"]["b"], "B");
    }

    #[test]
    fn merge_no_commands_override() {
        let base = json!({"commands": {"A": {}}, "qualifiers": {}});
        let overrides = json!({"qualifiers": {}});
        let merged = merge_mapping_data(&base, &overrides);
        assert!(merged["commands"]["A"].is_object());
    }

    #[test]
    fn merge_no_qualifiers_override() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {}}});
        let overrides = json!({"commands": {}});
        let merged = merge_mapping_data(&base, &overrides);
        assert!(merged["qualifiers"]["q1"].is_object());
    }

    // ---- validate_mapping_overrides_complete ----

    #[test]
    fn validate_complete_ok() {
        let base = json!({"commands": {"A": {}}, "qualifiers": {"q1": {}}});
        let overrides = json!({"commands": {"A": {}}, "qualifiers": {"q1": {}}});
        assert!(validate_mapping_overrides_complete(&base, &overrides).is_ok());
    }

    #[test]
    fn validate_complete_missing_commands() {
        let base = json!({"commands": {"A": {}, "B": {}}, "qualifiers": {}});
        let overrides = json!({"commands": {"A": {}}, "qualifiers": {}});
        let err = validate_mapping_overrides_complete(&base, &overrides).unwrap_err();
        assert!(err.contains("commands: B"));
    }

    #[test]
    fn validate_complete_missing_qualifiers() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {}, "q2": {}}});
        let overrides = json!({"commands": {}, "qualifiers": {"q1": {}}});
        let err = validate_mapping_overrides_complete(&base, &overrides).unwrap_err();
        assert!(err.contains("qualifiers: q2"));
    }

    #[test]
    fn validate_complete_missing_both() {
        let base = json!({"commands": {"A": {}}, "qualifiers": {"q1": {}}});
        let overrides = json!({"commands": {}, "qualifiers": {}});
        let err = validate_mapping_overrides_complete(&base, &overrides).unwrap_err();
        assert!(err.contains("commands: A"));
        assert!(err.contains("qualifiers: q1"));
    }

    #[test]
    fn validate_qualifier_with_valid_sub_keys() {
        let v = json!({
            "qualifiers": {
                "queue": {
                    "request_key_map": {"a": "A"},
                    "response_key_map": {"B": "b"}
                }
            }
        });
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn validate_commands_with_valid_entries() {
        let v = json!({
            "commands": {
                "DISPLAY QUEUE": {"qualifier": "queue"},
                "ALTER QMGR": {"qualifier": "qmgr"}
            }
        });
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn validate_only_commands_key() {
        let v = json!({"commands": {}});
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn validate_only_qualifiers_key() {
        let v = json!({"qualifiers": {}});
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn validate_empty_object() {
        let v = json!({});
        assert!(validate_mapping_overrides(&v).is_ok());
    }

    #[test]
    fn merge_qualifier_new_sub_key() {
        let base = json!({
            "commands": {},
            "qualifiers": {
                "q1": {
                    "request_key_map": {"a": "A"}
                }
            }
        });
        let overrides = json!({
            "qualifiers": {
                "q1": {
                    "response_key_map": {"B": "b"}
                }
            }
        });
        let merged = merge_mapping_data(&base, &overrides);
        assert_eq!(merged["qualifiers"]["q1"]["request_key_map"]["a"], "A");
        assert_eq!(merged["qualifiers"]["q1"]["response_key_map"]["B"], "b");
    }

    #[test]
    fn merge_qualifier_non_object_sub_value_ignored() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {"request_key_map": {"a": "A"}}}});
        let overrides = json!({"qualifiers": {"q1": {"request_key_map": "not_object"}}});
        let merged = merge_mapping_data(&base, &overrides);
        // Non-object sub-value should be ignored, original preserved
        assert_eq!(merged["qualifiers"]["q1"]["request_key_map"]["a"], "A");
    }

    #[test]
    fn merge_command_non_object_entry_ignored() {
        let base = json!({"commands": {"A": {"old": "1"}}, "qualifiers": {}});
        let overrides = json!({"commands": {"A": "not_object"}});
        let merged = merge_mapping_data(&base, &overrides);
        assert_eq!(merged["commands"]["A"]["old"], "1");
    }

    #[test]
    fn merge_qualifier_non_object_entry_ignored() {
        let base = json!({"commands": {}, "qualifiers": {"q1": {"request_key_map": {"a": "A"}}}});
        let overrides = json!({"qualifiers": {"q1": "not_object"}});
        let merged = merge_mapping_data(&base, &overrides);
        assert_eq!(merged["qualifiers"]["q1"]["request_key_map"]["a"], "A");
    }

    #[test]
    fn validate_complete_no_commands_in_base() {
        let base = json!({"qualifiers": {"q1": {}}});
        let overrides = json!({"qualifiers": {"q1": {}}});
        assert!(validate_mapping_overrides_complete(&base, &overrides).is_ok());
    }

    #[test]
    fn validate_complete_no_qualifiers_in_base() {
        let base = json!({"commands": {"A": {}}});
        let overrides = json!({"commands": {"A": {}}});
        assert!(validate_mapping_overrides_complete(&base, &overrides).is_ok());
    }

    // ---- replace_mapping_data ----

    #[test]
    fn replace_returns_clone() {
        let overrides = json!({"commands": {"X": {}}, "qualifiers": {}});
        let result = replace_mapping_data(&overrides);
        assert_eq!(result, overrides);
    }

    #[test]
    fn merge_into_base_missing_commands_key() {
        let base = json!({"qualifiers": {}});
        let overrides = json!({"commands": {"NEW_CMD": {"key": "value"}}});
        let result = merge_mapping_data(&base, &overrides);
        assert!(result.get("commands").unwrap().get("NEW_CMD").is_some());
    }

    #[test]
    fn merge_into_base_missing_qualifiers_key() {
        let base = json!({"commands": {}});
        let overrides = json!({"qualifiers": {"NEW_QUAL": {"sub": {"k": "v"}}}});
        let result = merge_mapping_data(&base, &overrides);
        assert!(result.get("qualifiers").unwrap().get("NEW_QUAL").is_some());
    }
}
