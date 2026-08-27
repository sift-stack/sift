use std::collections::HashMap;

use sift_rs::common::r#type::v1::ChannelDataType;

/// ROS 2 message types whose fields are collapsed into one nanosecond channel.
const TIME_MESSAGE_TYPES: [&str; 2] = ["builtin_interfaces/Time", "builtin_interfaces/Duration"];

/// Guards against malformed schemas with self-referential fixed nesting.
const MAX_FIELD_DEPTH: usize = 32;

/// Primitive type names the ROS 2 `.msg` grammar accepts. `duration` and
/// `time` are legacy ROS 1 spellings that parse but have no Sift mapping.
const PRIMITIVE_TYPES: [&str; 17] = [
    "bool", "byte", "char", "float32", "float64", "int8", "uint8", "int16", "uint16", "int32",
    "uint32", "int64", "uint64", "string", "wstring", "duration", "time",
];

/// The topic's schema cannot be decoded, so the topic is skipped.
#[derive(Debug)]
pub struct UnsupportedTopic(pub String);

impl std::fmt::Display for UnsupportedTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn unsupported<T>(msg: impl Into<String>) -> Result<T, UnsupportedTopic> {
    Err(UnsupportedTopic(msg.into()))
}

/// Maps a ROS 2 scalar type to its Sift channel type. Narrow integers widen to
/// 32-bit like the other import types. `byte` and `char` both map to `Uint32`:
/// ROS 2 defines them as unsigned 8-bit (octet and uint8).
fn ros2_to_sift_type(type_name: &str) -> Option<ChannelDataType> {
    Some(match type_name {
        "bool" => ChannelDataType::Bool,
        "int8" | "int16" | "int32" => ChannelDataType::Int32,
        "int64" => ChannelDataType::Int64,
        "uint8" | "uint16" | "uint32" | "byte" | "char" => ChannelDataType::Uint32,
        "uint64" => ChannelDataType::Uint64,
        "float32" => ChannelDataType::Float,
        "float64" => ChannelDataType::Double,
        "string" => ChannelDataType::String,
        _ => return None,
    })
}

/// A field type from a `.msg` definition.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldType {
    /// `None` for primitive types.
    pkg_name: Option<String>,
    type_name: String,
    is_array: bool,
    /// Element count for `[N]`, upper bound for `[<=N]`, `None` for `[]`.
    array_size: Option<usize>,
    is_upper_bound: bool,
}

impl FieldType {
    fn is_primitive(&self) -> bool {
        self.pkg_name.is_none()
    }

    /// Unbounded (`[]`) and bounded (`[<=N]`) arrays decode with a dynamic
    /// length, so they import whole rather than expanding into leaves.
    fn is_variable_array(&self) -> bool {
        self.is_array && (self.array_size.is_none() || self.is_upper_bound)
    }

    fn qualified_name(&self) -> String {
        match &self.pkg_name {
            Some(pkg) => format!("{pkg}/{}", self.type_name),
            None => self.type_name.clone(),
        }
    }

    fn is_time_message(&self) -> bool {
        TIME_MESSAGE_TYPES.contains(&self.qualified_name().as_str())
    }
}

#[derive(Debug, Clone)]
struct Field {
    ftype: FieldType,
    name: String,
}

#[derive(Debug, Clone)]
pub struct MsgDef {
    /// Qualified name (`<pkg>/<msg>`), used for cycle detection.
    qualified_name: String,
    fields: Vec<Field>,
}

/// A leaf field of a topic's message type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafField {
    /// Dot-delimited path within the decoded message, e.g. `orientation.x` or
    /// `orientation_covariance[0]`.
    pub field_path: String,
    pub data_type: ChannelDataType,
    /// Variable-cardinality field, imported whole rather than per element.
    pub complex: bool,
}

fn is_valid_package_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => (),
        _ => return false,
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return false;
    }
    !name.contains("__") && !name.ends_with('_')
}

fn is_valid_message_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => (),
        _ => return false,
    }
    name.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Parses one field type, e.g. `float64`, `string<=10`, `geometry_msgs/Point`,
/// `float64[9]`, `uint8[<=4]`. Unqualified message names resolve against
/// `context_pkg`, the package of the message being parsed.
fn parse_field_type(type_string: &str, context_pkg: &str) -> Result<FieldType, UnsupportedTopic> {
    let mut rest = type_string;
    let mut is_array = false;
    let mut array_size = None;
    let mut is_upper_bound = false;

    if rest.ends_with(']') {
        is_array = true;
        let open = match rest.rfind('[') {
            Some(idx) => idx,
            None => {
                return unsupported(format!("type '{type_string}' ends with ']' but has no '['"));
            }
        };
        let size_str = &rest[open + 1..rest.len() - 1];
        if !size_str.is_empty() {
            let digits = match size_str.strip_prefix("<=") {
                Some(d) => {
                    is_upper_bound = true;
                    d
                }
                None => size_str,
            };
            match digits.parse::<usize>() {
                Ok(n) if n > 0 => array_size = Some(n),
                _ => {
                    return unsupported(format!(
                        "the size of array type '{type_string}' must be an integer > 0, \
                         optionally prefixed with '<=' if it is only an upper bound"
                    ));
                }
            }
        }
        rest = &rest[..open];
    }

    // A bounded string keeps its base type; the bound does not affect import.
    if let Some(base) = bounded_string_base(rest) {
        return Ok(FieldType {
            pkg_name: None,
            type_name: base.to_string(),
            is_array,
            array_size,
            is_upper_bound,
        });
    }

    if PRIMITIVE_TYPES.contains(&rest) {
        return Ok(FieldType {
            pkg_name: None,
            type_name: rest.to_string(),
            is_array,
            array_size,
            is_upper_bound,
        });
    }

    let (pkg_name, type_name) = match rest.split('/').collect::<Vec<_>>()[..] {
        [pkg, name] => (pkg.to_string(), name.to_string()),
        [name] => (context_pkg.to_string(), name.to_string()),
        _ => return unsupported(format!("'{type_string}' is not a valid type name")),
    };

    if !is_valid_package_name(&pkg_name) {
        return unsupported(format!("'{pkg_name}' is not a valid package name"));
    }
    if !is_valid_message_name(&type_name) {
        return unsupported(format!("'{type_name}' is not a valid message name"));
    }

    Ok(FieldType {
        pkg_name: Some(pkg_name),
        type_name,
        is_array,
        array_size,
        is_upper_bound,
    })
}

/// Returns the base type of a bounded string (`string<=10`, `wstring<=10`).
fn bounded_string_base(type_string: &str) -> Option<&'static str> {
    for base in ["string", "wstring"] {
        if let Some(bound) = type_string
            .strip_prefix(base)
            .and_then(|r| r.strip_prefix("<="))
        {
            // The bound must be a positive integer, same as rosidl requires.
            if bound.parse::<usize>().is_ok_and(|n| n > 0) {
                return Some(base);
            }
        }
    }
    None
}

/// Parses one `.msg` definition body. Comments and constants
/// (`type NAME=value`) are skipped; only fields are returned.
fn parse_message_string(
    pkg_name: &str,
    msg_name: &str,
    body: &str,
) -> Result<MsgDef, UnsupportedTopic> {
    let body = body.replace('\t', " ");
    let mut fields = Vec::new();

    let lines: Vec<&str> = body.lines().skip_while(|l| l.starts_with('#')).collect();

    for line in lines {
        let mut line = line.trim_end();

        if let Some(idx) = line.find('#') {
            let before = &line[..idx];
            // An indented comment line annotates the previous field.
            if !before.is_empty() && before.trim().is_empty() {
                continue;
            }
            line = before.trim_end();
        }
        if line.is_empty() {
            continue;
        }

        let (type_string, rest) = match line.split_once(' ') {
            Some((t, r)) => (t, r.trim_start()),
            None => return unsupported(format!("'{line}' is not a valid field definition")),
        };
        if rest.is_empty() {
            return unsupported(format!("'{line}' is not a valid field definition"));
        }

        // A '=' after the name makes the line a constant, not a field.
        if rest.contains('=') {
            continue;
        }

        let field_name = rest.split(' ').next().unwrap_or_default();
        fields.push(Field {
            ftype: parse_field_type(type_string, pkg_name)?,
            name: field_name.to_string(),
        });
    }

    Ok(MsgDef {
        qualified_name: format!("{pkg_name}/{msg_name}"),
        fields,
    })
}

/// `builtin_interfaces/Time` and `Duration` are not carried in schema text.
/// Only a topic whose root type is one of them walks this definition; nested
/// occurrences collapse to a single channel before being resolved.
fn time_msgdef(qualified_name: &str) -> MsgDef {
    MsgDef {
        qualified_name: qualified_name.to_string(),
        fields: vec![
            Field {
                ftype: FieldType {
                    pkg_name: None,
                    type_name: "uint32".into(),
                    is_array: false,
                    array_size: None,
                    is_upper_bound: false,
                },
                name: "sec".into(),
            },
            Field {
                ftype: FieldType {
                    pkg_name: None,
                    type_name: "uint32".into(),
                    is_array: false,
                    array_size: None,
                    is_upper_bound: false,
                },
                name: "nanosec".into(),
            },
        ],
    }
}

fn is_separator(line: &str) -> bool {
    line.len() >= 3 && line.chars().all(|c| c == '=')
}

/// Parses a concatenated `ros2msg` schema into the root message definition and
/// every definition it references, keyed by both qualified and short name.
///
/// The text holds the topic's `.msg` definition followed by each definition it
/// references, separated by lines of `=` and headed by `MSG: <pkg>/<msg>`.
pub fn parse_schema_defs(
    schema_name: &str,
    schema_text: &str,
) -> Result<(MsgDef, HashMap<String, MsgDef>), UnsupportedTopic> {
    let mut msgdefs: HashMap<String, MsgDef> = HashMap::new();
    for name in TIME_MESSAGE_TYPES {
        msgdefs.insert(name.to_string(), time_msgdef(name));
    }

    // Blank lines carry no meaning and would otherwise split sections.
    let non_empty: Vec<&str> = schema_text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let mut cur_schema_name = schema_name.to_string();
    let mut section: Vec<&str> = Vec::new();
    let mut sections: Vec<Vec<&str>> = Vec::new();
    for line in non_empty {
        if is_separator(line.trim_end()) {
            sections.push(std::mem::take(&mut section));
        } else {
            section.push(line);
        }
    }
    sections.push(section);

    for section in sections {
        let mut body = Vec::new();
        for line in section {
            match line.trim().strip_prefix("MSG:") {
                Some(name) if !name.trim().is_empty() => {
                    cur_schema_name = name.trim().to_string();
                }
                _ => body.push(line),
            }
        }

        // "std_msgs/msg/String" names package "std_msgs" and message "String".
        let parts: Vec<&str> = cur_schema_name.split('/').collect();
        let pkg_name = parts.first().copied().unwrap_or_default();
        let msg_name = parts.last().copied().unwrap_or_default();
        let msgdef = parse_message_string(pkg_name, msg_name, &body.join("\n"))?;

        msgdefs.insert(cur_schema_name.clone(), msgdef.clone());
        msgdefs.insert(format!("{pkg_name}/{msg_name}"), msgdef);
    }

    let parts: Vec<&str> = schema_name.split('/').collect();
    let short_name = format!(
        "{}/{}",
        parts.first().copied().unwrap_or_default(),
        parts.last().copied().unwrap_or_default()
    );
    let root = msgdefs
        .get(schema_name)
        .or_else(|| msgdefs.get(&short_name))
        .cloned();

    match root {
        Some(root) => Ok((root, msgdefs)),
        None => unsupported("its schema does not define the root message"),
    }
}

fn resolve_or_unsupported<'a>(
    msgdefs: &'a HashMap<String, MsgDef>,
    ftype: &FieldType,
    label: &str,
) -> Result<&'a MsgDef, UnsupportedTopic> {
    match msgdefs.get(&ftype.qualified_name()) {
        Some(msgdef) => Ok(msgdef),
        None => unsupported(format!(
            "field '{label}' has unknown type '{}'",
            ftype.qualified_name()
        )),
    }
}

fn check_primitive_supported(type_name: &str, label: &str) -> Result<(), UnsupportedTopic> {
    if type_name == "wstring" {
        return unsupported(format!(
            "field '{label}' uses wstring, which the decoder does not implement"
        ));
    }
    if ros2_to_sift_type(type_name).is_none() {
        return unsupported(format!(
            "field '{label}' has unsupported type '{type_name}'"
        ));
    }
    Ok(())
}

/// A variable-cardinality field does not expand into leaves, but every element
/// is still decoded on import, so an undecodable type anywhere in its subtree
/// makes the whole topic unsupported.
fn check_ftype_decodable(
    msgdefs: &HashMap<String, MsgDef>,
    ftype: &FieldType,
    label: &str,
    visited: &[String],
    depth: usize,
) -> Result<(), UnsupportedTopic> {
    if ftype.is_primitive() {
        return check_primitive_supported(&ftype.type_name, label);
    }
    if ftype.is_time_message() {
        return Ok(());
    }
    let nested = resolve_or_unsupported(msgdefs, ftype, label)?;
    check_subtree_decodable(msgdefs, nested, visited, depth)
}

fn check_subtree_decodable(
    msgdefs: &HashMap<String, MsgDef>,
    msgdef: &MsgDef,
    visited: &[String],
    depth: usize,
) -> Result<(), UnsupportedTopic> {
    if depth > MAX_FIELD_DEPTH {
        return unsupported(format!(
            "its schema nests deeper than {MAX_FIELD_DEPTH} levels"
        ));
    }
    if visited.contains(&msgdef.qualified_name) {
        return Ok(());
    }
    let mut visited = visited.to_vec();
    visited.push(msgdef.qualified_name.clone());

    for field in &msgdef.fields {
        check_ftype_decodable(msgdefs, &field.ftype, &field.name, &visited, depth + 1)?;
    }
    Ok(())
}

/// Flattens the root message into importable leaves.
///
/// Scalar fields contribute their dot-delimited path, fixed-size arrays expand
/// with bracketed indexes, and variable-cardinality fields become one complex
/// leaf.
pub fn expand_message_fields(
    root: &MsgDef,
    msgdefs: &HashMap<String, MsgDef>,
) -> Result<Vec<LeafField>, UnsupportedTopic> {
    let mut leaves = Vec::new();
    walk(root, msgdefs, "", 0, &mut leaves)?;
    Ok(leaves)
}

fn walk(
    msgdef: &MsgDef,
    msgdefs: &HashMap<String, MsgDef>,
    prefix: &str,
    depth: usize,
    leaves: &mut Vec<LeafField>,
) -> Result<(), UnsupportedTopic> {
    if depth > MAX_FIELD_DEPTH {
        return unsupported(format!(
            "its schema nests deeper than {MAX_FIELD_DEPTH} levels"
        ));
    }

    for field in &msgdef.fields {
        let ftype = &field.ftype;
        let path = format!("{prefix}{}", field.name);

        if ftype.is_variable_array() {
            check_ftype_decodable(msgdefs, ftype, &path, &[], 0)?;
            leaves.push(LeafField {
                field_path: path,
                data_type: ChannelDataType::Bytes,
                complex: true,
            });
            continue;
        }

        let indexes: Vec<String> = match (ftype.is_array, ftype.array_size) {
            (true, Some(size)) => (0..size).map(|i| format!("[{i}]")).collect(),
            _ => vec![String::new()],
        };

        if ftype.is_primitive() {
            check_primitive_supported(&ftype.type_name, &path)?;
            let data_type =
                ros2_to_sift_type(&ftype.type_name).expect("checked as supported just above");
            leaves.extend(indexes.iter().map(|index| LeafField {
                field_path: format!("{path}{index}"),
                data_type,
                complex: false,
            }));
        } else if ftype.is_time_message() {
            leaves.extend(indexes.iter().map(|index| LeafField {
                field_path: format!("{path}{index}"),
                data_type: ChannelDataType::Int64,
                complex: false,
            }));
        } else {
            let nested = resolve_or_unsupported(msgdefs, ftype, &path)?;
            for index in &indexes {
                walk(
                    nested,
                    msgdefs,
                    &format!("{path}{index}."),
                    depth + 1,
                    leaves,
                )?;
            }
        }
    }
    Ok(())
}
