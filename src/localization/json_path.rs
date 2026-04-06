//! Dot-path selection (`App.Title`) with case-insensitive object keys (Json.NET `SelectToken` subset).

use serde_json::Value;

/// Returns a string leaf at `path` (segments separated by `.`), or `None` if missing or not a string.
pub fn select_string_token(value: &Value, path: &str) -> Option<String> {
    let mut cur = value;
    for segment in path.split('.') {
        if segment.is_empty() {
            return None;
        }
        cur = match cur {
            Value::Object(map) => map.iter().find(|(k, _)| k.eq_ignore_ascii_case(segment))?.1,
            _ => return None,
        };
    }
    match cur {
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() { None } else { Some(s.clone()) }
        }
        _ => None,
    }
}
