//! Runtime attribute mapping for MQSC <-> snake_case translations.

use std::collections::HashMap;

use serde_json::Value;

use crate::error::{MappingError, MappingIssue};
use crate::mapping_data::MAPPING_DATA;

/// Map request attributes from `snake_case` into MQSC parameter names.
pub fn map_request_attributes(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let qualifier_data = get_qualifier_data(qualifier, data);
    match qualifier_data {
        None => handle_unknown_qualifier(qualifier, attributes, "request", strict),
        Some(qd) => {
            let key_map = get_string_map(qd, "request_key_map");
            let key_value_map = get_key_value_map(qd, "request_key_value_map");
            let value_map = get_nested_string_map(qd, "request_value_map");
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
    }
}

/// Map response attributes from MQSC parameter names to `snake_case`.
pub fn map_response_attributes(
    qualifier: &str,
    attributes: &HashMap<String, Value>,
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<HashMap<String, Value>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let qualifier_data = get_qualifier_data(qualifier, data);
    match qualifier_data {
        None => handle_unknown_qualifier(qualifier, attributes, "response", strict),
        Some(qd) => {
            let key_map = get_string_map(qd, "response_key_map");
            let value_map = get_nested_string_map(qd, "response_value_map");
            let empty_kvm = HashMap::new();
            map_attributes(
                qualifier, attributes, &key_map, &empty_kvm, &value_map, "response", strict,
            )
        }
    }
}

/// Map a list of response objects from MQSC names to `snake_case`.
pub fn map_response_list(
    qualifier: &str,
    objects: &[HashMap<String, Value>],
    strict: bool,
    mapping_data: Option<&Value>,
) -> std::result::Result<Vec<HashMap<String, Value>>, MappingError> {
    let data = mapping_data.unwrap_or(&MAPPING_DATA);
    let qualifier_data = get_qualifier_data(qualifier, data);
    match qualifier_data {
        None => handle_unknown_qualifier_list(qualifier, objects, "response", strict),
        Some(qd) => {
            let key_map = get_string_map(qd, "response_key_map");
            let value_map = get_nested_string_map(qd, "response_value_map");
            let empty_kvm = HashMap::new();
            let mut mapped_objects = Vec::with_capacity(objects.len());
            let mut issues = Vec::new();
            for (object_index, attributes) in objects.iter().enumerate() {
                let (mapped, attr_issues) = map_attributes_internal(
                    qualifier,
                    attributes,
                    &key_map,
                    &empty_kvm,
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
    }
}

fn get_qualifier_data<'a>(qualifier: &str, data: &'a Value) -> Option<&'a Value> {
    data.get("qualifiers")?.get(qualifier)
}

fn get_string_map(qualifier_data: &Value, map_name: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Some(obj) = qualifier_data.get(map_name).and_then(Value::as_object) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                result.insert(k.clone(), s.to_owned());
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
        for (k, v) in obj {
            if let Some(inner) = v.as_object() {
                let mut inner_map = HashMap::new();
                for (ik, iv) in inner {
                    if let Some(s) = iv.as_str() {
                        inner_map.insert(ik.clone(), s.to_owned());
                    }
                }
                result.insert(k.clone(), inner_map);
            }
        }
    }
    result
}

type KeyValueMap = HashMap<String, HashMap<String, HashMap<String, String>>>;

fn get_key_value_map(qualifier_data: &Value, map_name: &str) -> KeyValueMap {
    let mut result = HashMap::new();
    if let Some(obj) = qualifier_data.get(map_name).and_then(Value::as_object) {
        for (k, v) in obj {
            if let Some(inner) = v.as_object() {
                let mut inner_map = HashMap::new();
                for (ik, iv) in inner {
                    if let Some(nested) = iv.as_object() {
                        let mut nested_map = HashMap::new();
                        for (nk, nv) in nested {
                            if let Some(s) = nv.as_str() {
                                nested_map.insert(nk.clone(), s.to_owned());
                            }
                        }
                        inner_map.insert(ik.clone(), nested_map);
                    }
                }
                result.insert(k.clone(), inner_map);
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
        match key_map.get(attr_name.as_str()) {
            None => {
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
            Some(mapped_key) => {
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
            }
        }
    }
    (mapped, issues)
}

fn map_value(
    qualifier: &str,
    attribute_name: &str,
    attribute_value: &Value,
    value_map: &HashMap<String, HashMap<String, String>>,
    direction: &str,
    object_index: Option<usize>,
) -> (Value, Vec<MappingIssue>) {
    let value_mappings = match value_map.get(attribute_name) {
        None => return (attribute_value.clone(), Vec::new()),
        Some(vm) => vm,
    };
    if let Some(str_val) = attribute_value.as_str() {
        match value_mappings.get(str_val) {
            None => (
                attribute_value.clone(),
                vec![MappingIssue {
                    direction: direction.into(),
                    reason: "unknown_value".into(),
                    attribute_name: attribute_name.into(),
                    attribute_value: Some(attribute_value.clone()),
                    object_index,
                    qualifier: Some(qualifier.into()),
                }],
            ),
            Some(mapped) => (Value::String(mapped.clone()), Vec::new()),
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
            match value_mappings.get(str_val) {
                None => {
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
                Some(mapped) => {
                    mapped_values.push(Value::String(mapped.clone()));
                }
            }
        } else {
            mapped_values.push(attr_value.clone());
        }
    }
    (Value::Array(mapped_values), issues)
}
