//! Deep-merge JSON objects with case-insensitive keys (Json.NET-style merge for localization).

use serde_json::{Map, Value};

/// Merges `overlay` into `dst` the way Newtonsoft `JObject.Merge` does for objects:
/// keys match case-insensitively; nested objects merge; non-object values replace.
pub fn merge_json(dst: &mut Value, overlay: Value) {
    match (dst, overlay) {
        (Value::Object(d), Value::Object(o)) => merge_objects(d, o),
        (dst, overlay) => *dst = overlay,
    }
}

fn merge_objects(dst: &mut Map<String, Value>, overlay: Map<String, Value>) {
    for (k, v) in overlay {
        let existing_key = dst.keys().find(|e| e.eq_ignore_ascii_case(&k)).cloned();
        if let Some(key) = existing_key {
            if let Some(slot) = dst.get_mut(&key) {
                merge_json(slot, v);
            }
        } else {
            dst.insert(k, v);
        }
    }
}
