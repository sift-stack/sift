//! Canonical, diff-stable snapshot of a test report tree for `export_test_report`.
//!
//! [`build`] lowers the flat proto types (report + steps + measurements) into a nested,
//! deterministically ordered tree and drops server-managed fields that a user cannot edit
//! (e.g. the report's derived `archived_date`), so a diff between two snapshots shows only
//! real, user-driven changes. Field names mirror the create/update tool inputs
//! (`string_expected`, `numeric_bounds`, `unit`) so a value can be copied from a snapshot
//! straight into an update call.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

use pbjson_types::Timestamp;
use serde::Serialize;
use sift_rs::{
    metadata::v1::{MetadataValue, metadata_value::Value as MetaInner},
    test_reports::v1::{
        TestMeasurement, TestMeasurementType, TestReport, TestStatus, TestStep, TestStepType,
        test_measurement,
    },
};

/// A snapshot plus the counts of what it contains, so the tool can report totals without
/// re-walking the tree.
#[derive(Debug)]
pub struct Export {
    pub report: ExportedReport,
    pub steps_exported: usize,
    pub measurements_exported: usize,
}

/// A metadata scalar, serialized as a bare JSON value (untagged) to match the tool inputs.
#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum MetaScalar {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExportedReport {
    pub test_report_id: String,
    pub name: String,
    pub status: String,
    pub test_system_name: String,
    pub test_case: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub serial_number: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub part_number: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub system_operator: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub run_id: String,
    pub is_archived: bool,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, MetaScalar>,
    pub steps: Vec<ExportedStep>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExportedStep {
    pub test_step_id: String,
    pub step_path: String,
    pub name: String,
    pub step_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ExportedErrorInfo>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, MetaScalar>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub measurements: Vec<ExportedMeasurement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<ExportedStep>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExportedErrorInfo {
    pub error_code: i32,
    pub error_message: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExportedMeasurement {
    pub measurement_id: String,
    pub name: String,
    pub measurement_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_bounds: Option<ExportedNumericBounds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_expected: Option<String>,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channel_names: Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, MetaScalar>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExportedNumericBounds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// Assemble the report, its steps, and its measurements into a canonical tree. Steps are nested
/// by `parent_step_id` and ordered by `step_path` (numeric segment order, so `10` sorts after
/// `2`); measurements are attached to their step and ordered by name then id. `steps_exported`
/// and `measurements_exported` count the fetched rows.
pub fn build(
    report: TestReport,
    steps: Vec<TestStep>,
    measurements: Vec<TestMeasurement>,
) -> Export {
    let steps_exported = steps.len();
    let measurements_exported = measurements.len();

    // Group measurements under their step, ordered deterministically.
    let mut by_step: HashMap<String, Vec<ExportedMeasurement>> = HashMap::new();
    for mut m in measurements {
        let step_id = std::mem::take(&mut m.test_step_id);
        by_step
            .entry(step_id)
            .or_default()
            .push(exported_measurement(m));
    }
    for v in by_step.values_mut() {
        v.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.measurement_id.cmp(&b.measurement_id))
        });
    }

    // Index steps by parent so children can be attached; roots have an empty parent.
    let mut children: HashMap<String, Vec<TestStep>> = HashMap::new();
    for mut s in steps {
        let parent = std::mem::take(&mut s.parent_step_id);
        children.entry(parent).or_default().push(s);
    }

    let mut roots = build_children("", &mut children, &mut by_step);
    // Defensive: never drop a step whose parent isn't present — surface it at the root rather
    // than losing it from an audit snapshot. Consistent server data leaves `children` empty here.
    // Ordering is handled by the final root sort below, so no need to order the orphan parents.
    let orphan_parents: Vec<String> = children.keys().cloned().collect();
    for parent in orphan_parents {
        roots.extend(build_children(&parent, &mut children, &mut by_step));
    }
    roots.sort_by(|a, b| cmp_step_path(&a.step_path, &b.step_path));

    Export {
        report: ExportedReport {
            test_report_id: report.test_report_id,
            name: report.name,
            status: status_name(report.status),
            test_system_name: report.test_system_name,
            test_case: report.test_case,
            start_time: ts_to_rfc3339(report.start_time.as_ref()),
            end_time: ts_to_rfc3339(report.end_time.as_ref()),
            serial_number: report.serial_number,
            part_number: report.part_number,
            system_operator: report.system_operator,
            run_id: report.run_id,
            is_archived: report.is_archived,
            metadata: metadata_map(report.metadata),
            steps: roots,
        },
        steps_exported,
        measurements_exported,
    }
}

fn build_children(
    parent_id: &str,
    children: &mut HashMap<String, Vec<TestStep>>,
    by_step: &mut HashMap<String, Vec<ExportedMeasurement>>,
) -> Vec<ExportedStep> {
    let Some(mut nodes) = children.remove(parent_id) else {
        return Vec::new();
    };
    nodes.sort_by(|a, b| cmp_step_path(&a.step_path, &b.step_path));
    nodes
        .into_iter()
        .map(|s| {
            let measurements = by_step.remove(&s.test_step_id).unwrap_or_default();
            let child_steps = build_children(&s.test_step_id, children, by_step);
            ExportedStep {
                measurements,
                steps: child_steps,
                error_info: s.error_info.map(|e| ExportedErrorInfo {
                    error_code: e.error_code,
                    error_message: e.error_message,
                }),
                start_time: ts_to_rfc3339(s.start_time.as_ref()),
                end_time: ts_to_rfc3339(s.end_time.as_ref()),
                metadata: metadata_map(s.metadata),
                step_type: step_type_name(s.step_type),
                status: status_name(s.status),
                test_step_id: s.test_step_id,
                step_path: s.step_path,
                name: s.name,
                description: s.description,
            }
        })
        .collect()
}

fn exported_measurement(m: TestMeasurement) -> ExportedMeasurement {
    let (numeric_value, string_value, boolean_value) = match m.value {
        Some(test_measurement::Value::NumericValue(v)) => (Some(v), None, None),
        Some(test_measurement::Value::StringValue(v)) => (None, Some(v), None),
        Some(test_measurement::Value::BooleanValue(v)) => (None, None, Some(v)),
        None => (None, None, None),
    };
    let (numeric_bounds, string_expected) = match m.bounds {
        Some(test_measurement::Bounds::NumericBounds(b)) => (
            Some(ExportedNumericBounds {
                min: b.min,
                max: b.max,
            }),
            None,
        ),
        Some(test_measurement::Bounds::StringBounds(b)) => (None, Some(b.expected_value)),
        None => (None, None),
    };
    ExportedMeasurement {
        measurement_id: m.measurement_id,
        name: m.name,
        measurement_type: measurement_type_name(m.measurement_type),
        numeric_value,
        string_value,
        boolean_value,
        unit: m.unit.map(|u| u.abbreviated_name).filter(|s| !s.is_empty()),
        numeric_bounds,
        string_expected,
        passed: m.passed,
        timestamp: ts_to_rfc3339(m.timestamp.as_ref()),
        description: m.description,
        channel_names: m.channel_names,
        metadata: metadata_map(m.metadata),
    }
}

/// Compare hierarchical step paths (`"1"`, `"1.10"`, `"2"`) segment by segment, numerically when
/// both segments parse as integers so `10` sorts after `2`.
fn cmp_step_path(a: &str, b: &str) -> Ordering {
    let mut ai = a.split('.');
    let mut bi = b.split('.');
    loop {
        match (ai.next(), bi.next()) {
            (Some(x), Some(y)) => {
                let ord = match (x.parse::<u64>(), y.parse::<u64>()) {
                    (Ok(nx), Ok(ny)) => nx.cmp(&ny),
                    _ => x.cmp(y),
                };
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

fn status_name(v: i32) -> String {
    TestStatus::try_from(v)
        .map(|e| e.as_str_name().to_string())
        .unwrap_or_else(|_| v.to_string())
}

fn step_type_name(v: i32) -> String {
    TestStepType::try_from(v)
        .map(|e| e.as_str_name().to_string())
        .unwrap_or_else(|_| v.to_string())
}

fn measurement_type_name(v: i32) -> String {
    TestMeasurementType::try_from(v)
        .map(|e| e.as_str_name().to_string())
        .unwrap_or_else(|_| v.to_string())
}

fn ts_to_rfc3339(ts: Option<&Timestamp>) -> Option<String> {
    let ts = ts?;
    let dt = chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)?;
    Some(dt.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true))
}

/// Lower proto metadata to a sorted map of scalars. Relation values (and any future non-scalar
/// kinds) are outside the user-editable scalar surface, so they are dropped.
fn metadata_map(values: Vec<MetadataValue>) -> BTreeMap<String, MetaScalar> {
    let mut map = BTreeMap::new();
    for mv in values {
        let Some(key) = mv.key else { continue };
        let scalar = match mv.value {
            Some(MetaInner::StringValue(s)) => MetaScalar::String(s),
            Some(MetaInner::NumberValue(n)) => MetaScalar::Number(n),
            Some(MetaInner::BooleanValue(b)) => MetaScalar::Boolean(b),
            _ => continue,
        };
        map.insert(key.name, scalar);
    }
    map
}
