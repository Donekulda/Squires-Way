//! Top-level child merge matching `ParserMerger.MergeTopLevel` list logic (without parse/format).
//!
//! Mirrors `IronyModManager.Parser.ParserMerger.MergeTopLevel` after both sides parse successfully:
//! `values` is `nodes.Values.First().Values`, target block is `targetNodes.Values.First().Values`.

/// Minimal script tree node aligned with `IronyModManager.Parser.Common.Parsers.Models.IScriptElement`
/// for the merge operation (key / nested values).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptElement {
    pub key: Option<String>,
    /// Clausewitz operator token (`IScriptElement.Operator` in Irony).
    pub op: Option<String>,
    pub value: Option<String>,
    pub is_simple_type: bool,
    pub children: Vec<ScriptElement>,
}

impl ScriptElement {
    pub fn new_key(key: impl Into<String>) -> Self {
        Self {
            key: Some(key.into()),
            op: None,
            value: None,
            is_simple_type: false,
            children: Vec::new(),
        }
    }
}

fn key_eq_ignore_case(el: &ScriptElement, target: &str) -> bool {
    el.key.as_deref().unwrap_or("").eq_ignore_ascii_case(target)
}

/// Merges target root children into `base_values` per Irony `MergeTopLevel`.
///
/// When `add_to_end` is true, appends `target_root_children` to the end of `base_values`.
/// Otherwise, finds the first element whose key equals `target_key` (case-insensitive; missing key
/// is treated as `""`), and inserts `target_root_children` at that index. If no key matches,
/// appends `target_root_children` instead.
pub fn merge_top_level_children(
    mut base_values: Vec<ScriptElement>,
    target_key: &str,
    target_root_children: Vec<ScriptElement>,
    add_to_end: bool,
) -> Vec<ScriptElement> {
    if add_to_end {
        base_values.extend(target_root_children);
        return base_values;
    }
    if let Some(index) = base_values
        .iter()
        .position(|p| key_eq_ignore_case(p, target_key))
    {
        base_values.splice(index..index, target_root_children);
    } else {
        base_values.extend(target_root_children);
    }
    base_values
}

#[cfg(test)]
mod tests {
    use super::*;

    fn el(key: &str) -> ScriptElement {
        ScriptElement::new_key(key)
    }

    #[test]
    fn add_to_end_appends() {
        let base = vec![el("a"), el("b")];
        let extra = vec![el("x"), el("y")];
        let out = merge_top_level_children(base, "ignored", extra, true);
        assert_eq!(
            out.iter().map(|e| e.key.as_deref()).collect::<Vec<_>>(),
            vec![Some("a"), Some("b"), Some("x"), Some("y")]
        );
    }

    #[test]
    fn insert_before_match_inserts_at_index_of_key() {
        let base = vec![el("first"), el("marker"), el("last")];
        let insert = vec![el("new1"), el("new2")];
        let out = merge_top_level_children(base, "marker", insert, false);
        assert_eq!(
            out.iter().map(|e| e.key.as_deref()).collect::<Vec<_>>(),
            vec![
                Some("first"),
                Some("new1"),
                Some("new2"),
                Some("marker"),
                Some("last")
            ]
        );
    }

    #[test]
    fn no_match_appends_like_add_range() {
        let base = vec![el("a")];
        let insert = vec![el("z")];
        let out = merge_top_level_children(base, "missing", insert, false);
        assert_eq!(
            out.iter().map(|e| e.key.as_deref()).collect::<Vec<_>>(),
            vec![Some("a"), Some("z")]
        );
    }

    #[test]
    fn key_compare_is_ascii_case_insensitive() {
        let base = vec![el("Foo")];
        let insert = vec![el("bar")];
        let out = merge_top_level_children(base, "foo", insert, false);
        assert_eq!(
            out.iter().map(|e| e.key.as_deref()).collect::<Vec<_>>(),
            vec![Some("bar"), Some("Foo")]
        );
    }

    #[test]
    fn none_key_matches_empty_target_key() {
        let bare = ScriptElement {
            key: None,
            op: None,
            value: None,
            is_simple_type: false,
            children: Vec::new(),
        };
        let base = vec![bare];
        let insert = vec![el("inserted")];
        let out = merge_top_level_children(base, "", insert, false);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].key.as_deref(), Some("inserted"));
        assert_eq!(out[1].key, None);
    }
}
