pub mod annotations;
pub mod artifacts;
pub mod assets;
pub mod calculated_channels;
pub mod channels;
pub mod data;
pub mod docs;
pub mod me;
pub mod remote_files;
pub mod report_templates;
pub mod reports;
pub mod rule_evaluation;
pub mod rules;
pub mod runs;
pub mod test_reports;
pub mod user_defined_functions;
pub mod users;
pub mod webhooks;

/// A catalog of mock patterns: canned responses, request matching, pagination and errors.
#[cfg(test)]
mod test;
