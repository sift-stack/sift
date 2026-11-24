use comfy_table::{ContentArrangement, Row, Table, presets::ASCII_FULL_CONDENSED};

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
