pub mod annotations;
pub mod assets;
pub mod calculated_channels;
pub mod campaigns;
pub mod channels;
pub mod data;
pub mod docs;
pub mod me;
pub mod report_templates;
pub mod reports;
pub mod rule_evaluation;
pub mod rules;
pub mod runs;
pub mod test_reports;
pub mod user_defined_functions;
pub mod users;

/// A test demonstrating a little bit of everything of how to leverage the mock API.
#[cfg(test)]
mod test;
