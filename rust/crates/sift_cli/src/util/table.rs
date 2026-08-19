use comfy_table::{ContentArrangement, Row, Table, presets::ASCII_FULL_CONDENSED};

/// Builds the table used for the text output of `get` subcommands.
///
/// Cells may carry OSC 8 hyperlinks and SGR styling, so this relies on
/// comfy-table's `custom_styling` feature to measure visible width rather than
/// byte length; without it the escape sequences inflate column widths and the
/// borders no longer line up. Content arrangement stays disabled so a cell is
/// never wrapped in the middle of an escape sequence.
pub fn new_table<H: Into<Row>>(headers: H) -> Table {
    let mut table = Table::new();
    table
        .load_preset(ASCII_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Disabled)
        .set_header(headers);
    table
}

#[cfg(test)]
mod test;
