use std::path::PathBuf;

use job_posting_schema::JobPosting;
use schemars::schema_for;
use serde_json::Value;

fn main() {
    let schema = schema_for!(JobPosting);
    let mut value = serde_json::to_value(&schema).expect("failed to convert schema to value");
    enforce_openai_object_rules(&mut value);
    strip_unsupported_formats(&mut value);
    let json = serde_json::to_string_pretty(&value)
        .expect("failed to serialize JSON schema");

    let output_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "schema.json"].iter().collect();
    std::fs::write(&output_path, format!("{json}\n"))
        .expect("failed to write schema.json");

    println!("wrote {}", output_path.display());
}

/// OpenAI's structured-output `response_format` requires every object schema
/// to set `additionalProperties: false` and to list every property in
/// `required` — optionality is expressed by allowing `null` in the property's
/// type rather than by omitting it from `required`. It also rejects `allOf`,
/// which schemars uses to attach a description to a `$ref`; we unwrap any
/// single-entry `allOf` and drop the sibling description (the referenced
/// definition carries its own).
fn enforce_openai_object_rules(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(items)) = map.get("allOf") {
                if items.len() == 1 {
                    if let Value::Object(inner) = &items[0] {
                        let inner = inner.clone();
                        map.remove("allOf");
                        map.remove("description");
                        for (k, v) in inner {
                            map.insert(k, v);
                        }
                    }
                }
            }

            let is_object_schema = map.get("type").and_then(Value::as_str) == Some("object")
                || map.contains_key("properties");
            if is_object_schema {
                if !map.contains_key("additionalProperties") {
                    map.insert("additionalProperties".to_string(), Value::Bool(false));
                }
                if let Some(Value::Object(properties)) = map.get("properties") {
                    let required: Vec<Value> = properties
                        .keys()
                        .map(|k| Value::String(k.clone()))
                        .collect();
                    map.insert("required".to_string(), Value::Array(required));
                }
            }
            for child in map.values_mut() {
                enforce_openai_object_rules(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                enforce_openai_object_rules(item);
            }
        }
        _ => {}
    }
}

/// OpenAI's structured-output `response_format` accepts only a narrow subset of
/// JSON Schema string formats — notably `uri` and friends are rejected even
/// though they're part of the spec. Strip any format outside that subset.
fn strip_unsupported_formats(value: &mut Value) {
    const OPENAI_FORMATS: &[&str] = &[
        "date-time", "date", "time", "duration",
        "email",
        "hostname",
        "ipv4", "ipv6",
        "uuid",
        "regex",
    ];
    match value {
        Value::Object(map) => {
            let drop = matches!(
                map.get("format").and_then(Value::as_str),
                Some(f) if !OPENAI_FORMATS.contains(&f)
            );
            if drop {
                map.remove("format");
            }
            for child in map.values_mut() {
                strip_unsupported_formats(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                strip_unsupported_formats(item);
            }
        }
        _ => {}
    }
}
