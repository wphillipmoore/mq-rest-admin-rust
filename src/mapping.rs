//! Runtime attribute mapping for MQSC <-> `snake_case` translations.
#![allow(clippy::implicit_hasher)]

use std::collections::HashMap;

use serde_json::Value;

use crate::error::{MappingError, MappingIssue};
use crate::mapping_data::MAPPING_DATA;

/// Map request attributes from `snake_case` into MQSC parameter names.
///
/// # Errors
///
/// Returns a `MappingError` in strict mode when an attribute name or value
/// cannot be mapped.
pub fn map_request_attributes(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let Some(qualifier_data) = get_qualifier_data(qualifier, data) else {
        return handle_unknown_qualifier(qualifier, attributes, "request", strict);
    };
    let key_map = get_string_map(qualifier_data, "request_key_map");
    let key_value_map = get_key_value_map(qualifier_data, "request_key_value_map");
    let value_map = get_nested_string_map(qualifier_data, "request_value_map");
    map_attributes(
        qualifier,
        attributes,
        &key_map,
        &key_value_map,
        &value_map,
        "request",
        strict,
    )
}

/// Map response attributes from MQSC parameter names to `snake_case`.
///
/// # Errors
///
/// Returns a `MappingError` in strict mode when an attribute name or value
/// cannot be mapped.
pub fn map_response_attributes(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let Some(qualifier_data) = get_qualifier_data(qualifier, data) else {
        return handle_unknown_qualifier(qualifier, attributes, "response", strict);
    };
    let key_map = get_string_map(qualifier_data, "response_key_map");
    let value_map = get_nested_string_map(qualifier_data, "response_value_map");
    let empty_key_value_map = HashMap::new();
    map_attributes(
        qualifier,
        attributes,
        &key_map,
        &empty_key_value_map,
        &value_map,
        "response",
        strict,
    )
}

/// Map a list of response objects from MQSC names to `snake_case`.
///
/// # Errors
///
/// Returns a `MappingError` in strict mode when an attribute name or value
/// cannot be mapped.
pub fn map_response_list(
    qualifier: &str,
    objects: &[HashMap<String, Value>],
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<Vec<HashMap<String, Value>>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let Some(qualifier_data) = get_qualifier_data(qualifier, data) else {
        return handle_unknown_qualifier_list(qualifier, objects, "response", strict);
    };
    let key_map = get_string_map(qualifier_data, "response_key_map");
    let value_map = get_nested_string_map(qualifier_data, "response_value_map");
    let empty_key_value_map = HashMap::new();
    let mut mapped_objects = Vec::with_capacity(objects.len());
    let mut issues = Vec::new();
    for (object_index, attributes) in objects.iter().enumerate() {
        let (mapped, attr_issues) = map_attributes_internal(
            qualifier,
            attributes,
            &key_map,
            &empty_key_value_map,
            &value_map,
            "response",
            Some(object_index),
        );
        mapped_objects.push(mapped);
        issues.extend(attr_issues);
    }
    if strict && !issues.is_empty() {
        return Err(MappingError::new(issues));
    }
    Ok(mapped_objects)
}

fn get_qualifier_data<'a>(qualifier: &str, data: &'a Value) -> Option<&'a Value> {
    data.get("qualifiers")?.get(qualifier)
}

fn get_string_map(qualifier_data: &Value, map_name: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Some(obj) = qualifier_data.get(map_name).and_then(Value::as_object) {
        for (key, value) in obj {
            if let Some(s) = value.as_str() {
                result.insert(key.clone(), s.to_owned());
            }
        }
    }
    result
}

fn get_nested_string_map(
    qualifier_data: &Value,
    map_name: &str,
) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();
    if let Some(obj) = qualifier_data.get(map_name).and_then(Value::as_object) {
        for (key, value) in obj {
            if let Some(inner) = value.as_object() {
                let mut inner_map = HashMap::new();
                for (inner_key, inner_value) in inner {
                    if let Some(s) = inner_value.as_str() {
                        inner_map.insert(inner_key.clone(), s.to_owned());
                    }
                }
                result.insert(key.clone(), inner_map);
            }
        }
    }
    result
}

type KeyValueMap = HashMap<String, HashMap<String, HashMap<String, String>>>;

fn get_key_value_map(qualifier_data: &Value, map_name: &str) -> KeyValueMap {
    let mut result = HashMap::new();
    if let Some(obj) = qualifier_data.get(map_name).and_then(Value::as_object) {
        for (key, value) in obj {
            if let Some(inner) = value.as_object() {
                let mut inner_map = HashMap::new();
                for (inner_key, inner_value) in inner {
                    if let Some(nested) = inner_value.as_object() {
                        let mut nested_map = HashMap::new();
                        for (nested_key, nested_value) in nested {
                            if let Some(s) = nested_value.as_str() {
                                nested_map.insert(nested_key.clone(), s.to_owned());
                            }
                        }
                        inner_map.insert(inner_key.clone(), nested_map);
                    }
                }
                result.insert(key.clone(), inner_map);
            }
        }
    }
    result
}

fn handle_unknown_qualifier(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    direction: &str,
    strict: bool,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    if !strict {
        return Ok(attributes.clone());
    }
    Err(MappingError::new(vec![MappingIssue {
        direction: direction.into(),
        reason: "unknown_qualifier".into(),
        attribute_name: "*".into(),
        attribute_value: None,
        object_index: None,
        qualifier: Some(qualifier.into()),
    }]))
}

fn handle_unknown_qualifier_list(
    qualifier: &str,
    objects: &[HashMap<String, Value>],
    direction: &str,
    strict: bool,
) -> std::result::Result<Vec<HashMap<String, Value>>, MappingError> {
    if !strict {
        return Ok(objects.to_vec());
    }
    Err(MappingError::new(vec![MappingIssue {
        direction: direction.into(),
        reason: "unknown_qualifier".into(),
        attribute_name: "*".into(),
        attribute_value: None,
        object_index: None,
        qualifier: Some(qualifier.into()),
    }]))
}

fn map_attributes(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    key_map: &HashMap<String, String>,
    key_value_map: &KeyValueMap,
    value_map: &HashMap<String, HashMap<String, String>>,
    direction: &str,
    strict: bool,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    let (mapped, issues) = map_attributes_internal(
        qualifier,
        attributes,
        key_map,
        key_value_map,
        value_map,
        direction,
        None,
    );
    if strict && !issues.is_empty() {
        return Err(MappingError::new(issues));
    }
    Ok(mapped)
}

fn map_attributes_internal(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    key_map: &HashMap<String, String>,
    key_value_map: &KeyValueMap,
    value_map: &HashMap<String, HashMap<String, String>>,
    direction: &str,
    object_index: Option<usize>,
) -> (HashMap<String, Value>, Vec<MappingIssue>) {
    let mut mapped = HashMap::new();
    let mut issues = Vec::new();
    for (attr_name, attr_value) in attributes {
        if direction == "request"
            && let Some(kv_map) = key_value_map.get(attr_name.as_str())
        {
            if let Some(str_val) = attr_value.as_str()
                && let Some(mapping) = kv_map.get(str_val)
                && let (Some(key), Some(value)) = (mapping.get("key"), mapping.get("value"))
            {
                mapped.insert(key.clone(), Value::String(value.clone()));
                continue;
            }
            issues.push(MappingIssue {
                direction: direction.into(),
                reason: "unknown_value".into(),
                attribute_name: attr_name.clone(),
                attribute_value: Some(attr_value.clone()),
                object_index,
                qualifier: Some(qualifier.into()),
            });
            mapped.insert(attr_name.clone(), attr_value.clone());
            continue;
        }
        if let Some(mapped_key) = key_map.get(attr_name.as_str()) {
            let (mapped_value, value_issues) = map_value(
                qualifier,
                attr_name,
                attr_value,
                value_map,
                direction,
                object_index,
            );
            mapped.insert(mapped_key.clone(), mapped_value);
            issues.extend(value_issues);
        } else {
            issues.push(MappingIssue {
                direction: direction.into(),
                reason: "unknown_key".into(),
                attribute_name: attr_name.clone(),
                attribute_value: Some(attr_value.clone()),
                object_index,
                qualifier: Some(qualifier.into()),
            });
            mapped.insert(attr_name.clone(), attr_value.clone());
        }
    }
    (mapped, issues)
}

#[allow(clippy::option_if_let_else)]
fn map_value(
    qualifier: &str,
    attribute_name: &str,
    attribute_value: &Value,
    value_map: &HashMap<String, HashMap<String, String>>,
    direction: &str,
    object_index: Option<usize>,
) -> (Value, Vec<MappingIssue>) {
    let Some(value_mappings) = value_map.get(attribute_name) else {
        return (attribute_value.clone(), Vec::new());
    };
    if let Some(str_val) = attribute_value.as_str() {
        if let Some(mapped) = value_mappings.get(str_val) {
            (Value::String(mapped.clone()), Vec::new())
        } else {
            (
                attribute_value.clone(),
                vec![MappingIssue {
                    direction: direction.into(),
                    reason: "unknown_value".into(),
                    attribute_name: attribute_name.into(),
                    attribute_value: Some(attribute_value.clone()),
                    object_index,
                    qualifier: Some(qualifier.into()),
                }],
            )
        }
    } else if let Some(arr) = attribute_value.as_array() {
        map_value_list(
            qualifier,
            attribute_name,
            arr,
            value_mappings,
            direction,
            object_index,
        )
    } else {
        (attribute_value.clone(), Vec::new())
    }
}

fn map_value_list(
    qualifier: &str,
    attribute_name: &str,
    attribute_values: &[Value],
    value_mappings: &HashMap<String, String>,
    direction: &str,
    object_index: Option<usize>,
) -> (Value, Vec<MappingIssue>) {
    let mut mapped_values = Vec::with_capacity(attribute_values.len());
    let mut issues = Vec::new();
    for attr_value in attribute_values {
        if let Some(str_val) = attr_value.as_str() {
            if let Some(mapped) = value_mappings.get(str_val) {
                mapped_values.push(Value::String(mapped.clone()));
            } else {
                issues.push(MappingIssue {
                    direction: direction.into(),
                    reason: "unknown_value".into(),
                    attribute_name: attribute_name.into(),
                    attribute_value: Some(attr_value.clone()),
                    object_index,
                    qualifier: Some(qualifier.into()),
                });
                mapped_values.push(attr_value.clone());
            }
        } else {
            mapped_values.push(attr_value.clone());
        }
    }
    (Value::Array(mapped_values), issues)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_mapping_data() -> Value {
        json!({
            "qualifiers": {
                "testq": {
                    "request_key_map": {
                        "description": "DESCR",
                        "max_depth": "MAXDEPTH"
                    },
                    "request_value_map": {
                        "description": {
                            "default_value": "DEFVAL"
                        }
                    },
                    "request_key_value_map": {
                        "queue_type": {
                            "local": {"key": "QTYPE", "value": "QLOCAL"},
                            "remote": {"key": "QTYPE", "value": "QREMOTE"}
                        }
                    },
                    "response_key_map": {
                        "DESCR": "description",
                        "MAXDEPTH": "max_depth"
                    },
                    "response_value_map": {
                        "DESCR": {
                            "DEFVAL": "default_value"
                        }
                    }
                }
            }
        })
    }

    // ---- map_request_attributes ----

    #[test]
    fn map_request_key_mapping() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("description".into(), json!("hello"));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("DESCR").unwrap(), &json!("hello"));
    }

    #[test]
    fn map_request_value_mapping() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("description".into(), json!("default_value"));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("DESCR").unwrap(), &json!("DEFVAL"));
    }

    #[test]
    fn map_request_key_value_mapping() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("queue_type".into(), json!("local"));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("QTYPE").unwrap(), &json!("QLOCAL"));
    }

    #[test]
    fn map_request_key_value_unknown_value() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("queue_type".into(), json!("bogus"));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("queue_type").unwrap(), &json!("bogus"));
    }

    #[test]
    fn map_request_key_value_non_string_value() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("queue_type".into(), json!(42));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("queue_type").unwrap(), &json!(42));
    }

    #[test]
    fn map_request_unknown_key_non_strict() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("unknown_attr".into(), json!("val"));
        let result = map_request_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("unknown_attr").unwrap(), &json!("val"));
    }

    #[test]
    fn map_request_unknown_key_strict() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("unknown_attr".into(), json!("val"));
        let result = map_request_attributes("testq", &attrs, true, Some(&data));
        assert!(result.is_err());
    }

    #[test]
    fn map_request_unknown_qualifier_non_strict() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("foo".into(), json!("bar"));
        let result = map_request_attributes("noexist", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("foo").unwrap(), &json!("bar"));
    }

    #[test]
    fn map_request_unknown_qualifier_strict() {
        let data = test_mapping_data();
        let attrs = HashMap::new();
        let result = map_request_attributes("noexist", &attrs, true, Some(&data));
        assert!(result.is_err());
    }

    // ---- map_response_attributes ----

    #[test]
    fn map_response_key_and_value() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("DESCR".into(), json!("DEFVAL"));
        let result = map_response_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("description").unwrap(), &json!("default_value"));
    }

    #[test]
    fn map_response_unknown_value_non_strict() {
        let data = test_mapping_data();
        let mut attrs = HashMap::new();
        attrs.insert("DESCR".into(), json!("NOMATCH"));
        let result = map_response_attributes("testq", &attrs, false, Some(&data)).unwrap();
        assert_eq!(result.get("description").unwrap(), &json!("NOMATCH"));
    }

    #[test]
    fn map_response_unknown_qualifier_strict() {
        let data = test_mapping_data();
        let attrs = HashMap::new();
        let result = map_response_attributes("noexist", &attrs, true, Some(&data));
        assert!(result.is_err());
    }

    // ---- map_response_list ----

    #[test]
    fn map_response_list_multiple_objects() {
        let data = test_mapping_data();
        let obj1 = {
            let mut m = HashMap::new();
            m.insert("DESCR".into(), json!("DEFVAL"));
            m
        };
        let obj2 = {
            let mut m = HashMap::new();
            m.insert("MAXDEPTH".into(), json!("5000"));
            m
        };
        let result = map_response_list("testq", &[obj1, obj2], false, Some(&data)).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0].get("description").unwrap(),
            &json!("default_value")
        );
        assert_eq!(result[1].get("max_depth").unwrap(), &json!("5000"));
    }

    #[test]
    fn map_response_list_unknown_qualifier_non_strict() {
        let data = test_mapping_data();
        let obj = HashMap::new();
        let result = map_response_list("noexist", &[obj], false, Some(&data)).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn map_response_list_unknown_qualifier_strict() {
        let data = test_mapping_data();
        let obj = HashMap::new();
        let result = map_response_list("noexist", &[obj], true, Some(&data));
        assert!(result.is_err());
    }

    #[test]
    fn map_response_list_strict_with_unknown_keys() {
        let data = test_mapping_data();
        let mut obj = HashMap::new();
        obj.insert("UNKNOWN".into(), json!("val"));
        let result = map_response_list("testq", &[obj], true, Some(&data));
        assert!(result.is_err());
    }

    // ---- Private helper: map_value ----

    #[test]
    fn map_value_string_hit() {
        let mut value_map = HashMap::new();
        let mut inner = HashMap::new();
        inner.insert("YES".into(), "yes".into());
        value_map.insert("attr".into(), inner);
        let (result, issues) = map_value("q", "attr", &json!("YES"), &value_map, "response", None);
        assert_eq!(result, json!("yes"));
        assert!(issues.is_empty());
    }

    #[test]
    fn map_value_string_miss() {
        let mut value_map = HashMap::new();
        let mut inner = HashMap::new();
        inner.insert("YES".into(), "yes".into());
        value_map.insert("attr".into(), inner);
        let (result, issues) = map_value("q", "attr", &json!("NOPE"), &value_map, "response", None);
        assert_eq!(result, json!("NOPE"));
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn map_value_array() {
        let mut value_map = HashMap::new();
        let mut inner = HashMap::new();
        inner.insert("A".into(), "a".into());
        value_map.insert("attr".into(), inner);
        let arr = json!(["A", "B"]);
        let (result, issues) = map_value("q", "attr", &arr, &value_map, "response", Some(0));
        let arr_result = result.as_array().unwrap();
        assert_eq!(arr_result[0], json!("a"));
        assert_eq!(arr_result[1], json!("B"));
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn map_value_non_string_passthrough() {
        let mut value_map = HashMap::new();
        value_map.insert("attr".into(), HashMap::new());
        let (result, issues) = map_value("q", "attr", &json!(42), &value_map, "response", None);
        assert_eq!(result, json!(42));
        assert!(issues.is_empty());
    }

    #[test]
    fn map_value_no_entry() {
        let value_map = HashMap::new();
        let (result, issues) = map_value("q", "attr", &json!("val"), &value_map, "response", None);
        assert_eq!(result, json!("val"));
        assert!(issues.is_empty());
    }

    // ---- map_value_list ----

    #[test]
    fn map_value_list_mixed_types() {
        let mut mappings = HashMap::new();
        mappings.insert("A".into(), "mapped_a".into());
        let values = vec![json!("A"), json!(123), json!("UNKNOWN")];
        let (result, issues) = map_value_list("q", "attr", &values, &mappings, "response", None);
        let arr = result.as_array().unwrap();
        assert_eq!(arr[0], json!("mapped_a"));
        assert_eq!(arr[1], json!(123));
        assert_eq!(arr[2], json!("UNKNOWN"));
        assert_eq!(issues.len(), 1);
    }

    // ---- get_string_map / get_nested_string_map / get_key_value_map ----

    #[test]
    fn get_string_map_missing_field() {
        let data = json!({});
        let result = get_string_map(&data, "nonexistent");
        assert!(result.is_empty());
    }

    #[test]
    fn get_string_map_non_string_values_ignored() {
        let data = json!({"test_map": {"a": "b", "c": 42}});
        let result = get_string_map(&data, "test_map");
        assert_eq!(result.len(), 1);
        assert_eq!(result["a"], "b");
    }

    #[test]
    fn get_nested_string_map_missing() {
        let data = json!({});
        let result = get_nested_string_map(&data, "nonexistent");
        assert!(result.is_empty());
    }

    #[test]
    fn get_nested_string_map_non_object_inner_ignored() {
        let data = json!({"vm": {"key": "not_an_object"}});
        let result = get_nested_string_map(&data, "vm");
        assert!(result.is_empty());
    }

    #[test]
    fn get_key_value_map_missing() {
        let data = json!({});
        let result = get_key_value_map(&data, "nonexistent");
        assert!(result.is_empty());
    }

    #[test]
    fn get_key_value_map_valid() {
        let data = json!({
            "kvm": {
                "queue_type": {
                    "local": {"key": "QTYPE", "value": "QLOCAL"}
                }
            }
        });
        let result = get_key_value_map(&data, "kvm");
        assert_eq!(result["queue_type"]["local"]["key"], "QTYPE");
    }

    #[test]
    fn get_key_value_map_non_object_inner_ignored() {
        let data = json!({"kvm": {"key": "string_not_object"}});
        let result = get_key_value_map(&data, "kvm");
        assert!(result.is_empty());
    }

    #[test]
    fn get_key_value_map_non_object_nested_ignored() {
        let data = json!({"kvm": {"key": {"inner": "not_object"}}});
        let result = get_key_value_map(&data, "kvm");
        assert!(result["key"].is_empty());
    }

    // ---- Uses MAPPING_DATA (integration-level) ----

    #[test]
    fn map_request_with_bundled_data() {
        let mut attrs = HashMap::new();
        attrs.insert("description".into(), json!("test"));
        let result = map_request_attributes("queue", &attrs, false, None).unwrap();
        assert!(result.contains_key("DESCR"));
    }

    #[test]
    fn map_response_with_bundled_data() {
        let mut attrs = HashMap::new();
        attrs.insert("DESCR".into(), json!("test"));
        let result = map_response_attributes("queue", &attrs, false, None).unwrap();
        assert!(result.contains_key("description"));
    }
}
