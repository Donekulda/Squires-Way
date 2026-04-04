//! Clausewitz-style formatting for [`super::merge_top_level::ScriptElement`] (Irony `CodeParser.FormatCode` subset).

use crate::merging::merge_top_level::ScriptElement;

/// Formats a single element (no trailing newline). Mirrors Irony `CodeParser.FormatCode` structure
/// for common `key = value` and `key = { ... }` blocks (inline-operator branches omitted).
pub fn format_script_element(element: &ScriptElement, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    if element.is_simple_type {
        return format_simple(element, &pad);
    }
    format_complex(element, indent, &pad)
}

fn format_simple(element: &ScriptElement, pad: &str) -> String {
    let key = element.key.as_deref().unwrap_or("");
    if let Some(v) = element.value.as_ref() {
        if v.is_empty() {
            return format!("{pad}{key}");
        }
        if let Some(op) = element.op.as_deref().filter(|s| !s.is_empty()) {
            format!("{pad}{key} {op} {v}")
        } else {
            format!("{pad}{key} {v}")
        }
    } else {
        format!("{pad}{key}")
    }
}

fn format_complex(element: &ScriptElement, indent: usize, pad: &str) -> String {
    let key = element.key.as_deref().unwrap_or("");
    let mut out = String::new();
    if let Some(op) = element.op.as_deref().filter(|s| !s.is_empty()) {
        out.push_str(pad);
        out.push_str(key);
        out.push(' ');
        out.push_str(op);
        out.push_str(" {\n");
    } else {
        out.push_str(pad);
        out.push_str(key);
        out.push_str(" {\n");
    }
    for child in &element.children {
        out.push_str(&format_script_element(child, indent + 1));
        out.push('\n');
    }
    out.push_str(pad);
    out.push('}');
    out
}

/// Joins root-level elements the way Irony does: `AppendLine(FormatCode(node))` for each.
pub fn format_root_elements(roots: &[ScriptElement]) -> String {
    let mut sb = String::new();
    for (i, node) in roots.iter().enumerate() {
        if i > 0 {
            sb.push('\n');
        }
        sb.push_str(&format_script_element(node, 0));
    }
    if !sb.is_empty() {
        sb.push('\n');
    }
    sb
}
