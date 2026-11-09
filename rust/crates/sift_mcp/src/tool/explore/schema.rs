use std::collections::BTreeSet;

use rmcp::{ErrorData, model::CallToolResult};
use serde_json::{Map, Value, json};

use crate::{error, service::declarative::ChartSchema};

pub(super) fn result(schema: &ChartSchema, names: Option<&[String]>) -> error::McpResult {
    let Some(names) = names else {
        let definitions: Map<String, Value> = schema
            .definitions
            .iter()
            .map(|(name, definition)| {
                let description = definition["description"].as_str().unwrap_or_default();
                (name.clone(), json!(summary(description)))
            })
            .collect();
        let description = if schema.description.trim().is_empty() {
            &schema.title
        } else {
            &schema.description
        };
        // Collect sorted, deduplicated $defs names referenced by root constraints.
        let mut references: BTreeSet<&str> = BTreeSet::new();
        for value in schema.root.values() {
            collect_references(value, &mut references);
        }
        return Ok(CallToolResult::structured(json!({
            "$id": schema.id,
            "title": schema.title,
            "summary": summary(description),
            "root": schema.root,
            "references": references,
            "definitions": definitions,
            "save_command": "sift-cli get chart-schema -o <path>",
        })));
    };

    let mut definitions = Map::new();
    let mut references = BTreeSet::new();
    for name in names {
        let Some(definition) = schema.definitions.get(name) else {
            let mut matches: Vec<_> = schema
                .definitions
                .keys()
                .map(|candidate| {
                    (
                        edit_distance(&name.to_lowercase(), &candidate.to_lowercase()),
                        candidate,
                    )
                })
                .collect();
            matches.sort_unstable();
            let close_matches: Vec<_> = matches
                .into_iter()
                .filter(|(distance, _)| *distance <= (name.chars().count() / 3).max(2))
                .take(3)
                .map(|(_, candidate)| candidate.as_str())
                .collect();
            let suggestions = if close_matches.is_empty() {
                "none. Call get_chart_schema without parameters for the index".to_owned()
            } else {
                close_matches.join(", ")
            };
            return Err(ErrorData::invalid_params(
                format!("Unknown chart schema definition {name:?}. Close matches: {suggestions}."),
                Some(json!({"name": name, "close_matches": close_matches})),
            ));
        };
        collect_references(definition, &mut references);
        definitions.insert(name.clone(), definition.clone());
    }
    Ok(CallToolResult::structured(json!({
        "$id": schema.id,
        "title": schema.title,
        "definitions": definitions,
        "references": references,
    })))
}

fn summary(description: &str) -> String {
    let mut summary = description.split_whitespace().collect::<Vec<_>>().join(" ");
    let end = summary.char_indices().find_map(|(index, character)| {
        let end = index + character.len_utf8();
        (matches!(character, '.' | '!' | '?')
            && summary[end..].starts_with(' ')
            && !["e.g.", "i.e.", "etc."]
                .iter()
                .any(|abbreviation| summary[..end].ends_with(abbreviation)))
        .then_some(end)
    });
    if let Some(end) = end {
        summary.truncate(end);
    }
    summary
}

fn collect_references<'a>(value: &'a Value, references: &mut BTreeSet<&'a str>) {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str)
                && let Some(path) = reference.strip_prefix("#/$defs/")
                && let Some(name) = path.split('/').next().filter(|name| !name.is_empty())
            {
                references.insert(name);
            }
            for value in object.values() {
                collect_references(value, references);
            }
        }
        Value::Array(array) => {
            for value in array {
                collect_references(value, references);
            }
        }
        _ => {}
    }
}

fn edit_distance(left: &str, right: &str) -> usize {
    let right: Vec<_> = right.chars().collect();
    let mut row: Vec<_> = (0..=right.len()).collect();
    for (i, left) in left.chars().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, right) in right.iter().enumerate() {
            let previous = row[j + 1];
            row[j + 1] = (row[j] + 1)
                .min(previous + 1)
                .min(diagonal + usize::from(left != *right));
            diagonal = previous;
        }
    }
    row[right.len()]
}

#[cfg(test)]
mod test {
    #[test]
    fn summary_preserves_abbreviations() {
        for abbreviation in ["e.g.", "i.e.", "etc."] {
            assert_eq!(
                super::summary(&format!("Use {abbreviation} a channel ID. More text.")),
                format!("Use {abbreviation} a channel ID.")
            );
        }
    }
}
