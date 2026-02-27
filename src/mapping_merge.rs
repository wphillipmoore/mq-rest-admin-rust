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
                for (ek, ev) in entry_obj {
                    existing.insert(ek.clone(), ev.clone());
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
                            for (k, v) in sub_obj {
                                existing_sub.insert(k.clone(), v.clone());
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
