use super::new_table;
use crate::util::tty::{hyperlink, link_style};

/// Strips OSC and SGR escape sequences so only the visible characters remain.
fn visible(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\x1b' {
            out.push(c);
            continue;
        }
        match chars.next() {
            // OSC, terminated by BEL or ESC \
            Some(']') => {
                while let Some(c) = chars.next() {
                    if c == '\x07' {
                        break;
                    }
                    if c == '\x1b' {
                        chars.next();
                        break;
                    }
                }
            }
            // CSI, terminated by a final byte in the alphabetic range
            Some('[') => {
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[test]
fn borders_stay_aligned_when_a_cell_holds_a_styled_hyperlink() {
    let mut table = new_table(vec!["ID", "Name"]);
    table.add_row(vec![
        "c6a9e2b8-0000-4000-8000-1234567890ab".to_string(),
        hyperlink(
            &link_style("engine"),
            "https://app.siftstack.com/explore?method=single&assets=engine",
        ),
    ]);
    table.add_row(vec![
        "7f13aa20-1111-4000-8000-abcdefabcdef".to_string(),
        "avionics-bench-3".to_string(),
    ]);

    let rendered = table.to_string();
    let widths: Vec<usize> = rendered
        .lines()
        .map(|line| visible(line).chars().count())
        .collect();

    assert!(!widths.is_empty(), "table rendered no lines");
    assert!(
        widths.iter().all(|w| *w == widths[0]),
        "every line should have the same visible width, got {widths:?}:\n{}",
        rendered.replace('\x1b', "<ESC>")
    );
}

#[test]
fn hyperlink_cell_keeps_its_escape_sequence_intact() {
    let url = "https://app.siftstack.com/explore?method=single&assets=engine";
    let mut table = new_table(vec!["Name"]);
    table.add_row(vec![hyperlink("engine", url)]);

    let rendered = table.to_string();
    assert!(rendered.contains(&format!("\x1b]8;;{url}\x1b\\engine\x1b]8;;\x1b\\")));
}
