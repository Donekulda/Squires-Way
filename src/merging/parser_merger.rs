//! End-to-end `ParserMerger.MergeTopLevel` using [`jomini::TextTape`] for parse + [`format_code`] for emit.
//!
//! Irony uses `CodeParser.ParseScriptWithoutValidation` and `FormatCode`; we approximate the same
//! pipeline with jomini’s UTF-8 reader so UTF-8 mod files round-trip for merge. Formatting may
//! differ slightly from Irony for edge cases (inline operators, arrays).

use jomini::text::{ObjectReader, Operator, ScalarReader, ValueReader};
use jomini::{TextTape, Utf8Encoding};

use super::format_code::format_root_elements;
use super::merge_top_level::{ScriptElement, merge_top_level_children};

fn join_lines(lines: &[String]) -> String {
    lines.join("\n")
}

/// Full `ParserMerger.MergeTopLevel` pipeline: parse base and target, merge first root block’s
/// children, re-emit. Returns empty string on parse failure or empty roots (matches C#).
///
/// `file_name` is accepted for API parity with Irony; jomini does not use it today.
pub fn merge_top_level(
    code_lines: &[String],
    _file_name: &str,
    target: &str,
    target_code_lines: &[String],
    add_to_end: bool,
) -> String {
    let base_text = join_lines(code_lines);
    let target_text = join_lines(target_code_lines);
    merge_top_level_str(&base_text, target, &target_text, add_to_end)
}

/// Like [`merge_top_level`] but accepts full file text (still UTF-8, `\n` line endings).
pub fn merge_top_level_str(
    base_code: &str,
    target: &str,
    target_code: &str,
    add_to_end: bool,
) -> String {
    let Ok(base_tape) = TextTape::from_slice(base_code.as_bytes()) else {
        return String::new();
    };
    let Ok(target_tape) = TextTape::from_slice(target_code.as_bytes()) else {
        return String::new();
    };
    let base_reader = base_tape.utf8_reader();
    let target_reader = target_tape.utf8_reader();
    let mut base_roots = match object_to_root_elements(base_reader) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let target_roots = match object_to_root_elements(target_reader) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    if base_roots.is_empty() || target_roots.is_empty() {
        return String::new();
    }
    let target_children = target_roots[0].children.clone();
    let base_first = &mut base_roots[0];
    let merged = merge_top_level_children(
        std::mem::take(&mut base_first.children),
        target,
        target_children,
        add_to_end,
    );
    base_first.children = merged;
    format_root_elements(&base_roots)
}

fn object_to_root_elements(
    reader: ObjectReader<'_, '_, Utf8Encoding>,
) -> Result<Vec<ScriptElement>, ()> {
    let mut out = Vec::new();
    for (key, op, value) in reader.fields() {
        out.push(field_to_element(key, op, value)?);
    }
    Ok(out)
}

fn field_to_element(
    key: ScalarReader<'_, Utf8Encoding>,
    op: Option<Operator>,
    value: ValueReader<'_, '_, Utf8Encoding>,
) -> Result<ScriptElement, ()> {
    let key_str = key.read_string();
    let op_str = op.map(|o| o.symbol().to_string());

    if let Ok(inner) = value.read_object() {
        let children = object_to_root_elements(inner)?;
        return Ok(ScriptElement {
            key: Some(key_str),
            op: op_str,
            value: None,
            is_simple_type: false,
            children,
        });
    }

    if let Ok(s) = value.read_string() {
        return Ok(ScriptElement {
            key: Some(key_str),
            op: op_str,
            value: Some(s),
            is_simple_type: true,
            children: Vec::new(),
        });
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &str) -> Vec<String> {
        s.lines().map(String::from).collect()
    }

    #[test]
    fn merge_inserts_before_matching_child_key() {
        let base = lines(
            r#"block = {
    first = 1
    marker = 2
    last = 3
}"#,
        );
        let target = lines(
            r#"block = {
    new1 = 10
    new2 = 20
}"#,
        );
        let out = merge_top_level(&base, "test.txt", "marker", &target, false);
        assert!(
            out.contains("new1"),
            "expected inserted keys in output:\n{out}"
        );
        assert!(out.contains("marker"));
        let pos_new = out.find("new1").unwrap();
        let pos_marker = out.find("marker").unwrap();
        assert!(
            pos_new < pos_marker,
            "inserts should appear before marker:\n{out}"
        );
    }

    #[test]
    fn merge_add_to_end_appends_target_children() {
        let base = lines(
            r#"b = {
    a = 1
}"#,
        );
        let target = lines(
            r#"b = {
    z = 9
}"#,
        );
        let out = merge_top_level(&base, "t.txt", "ignored", &target, true);
        let pos_a = out.find("a").expect("base child");
        let pos_z = out.find("z").expect("merged child");
        assert!(
            pos_a < pos_z,
            "add_to_end should place target children after base children:\n{out}"
        );
    }

    #[test]
    fn parse_failure_returns_empty() {
        let out = merge_top_level_str("{", "k", "}", false);
        assert!(out.is_empty());
    }
}
