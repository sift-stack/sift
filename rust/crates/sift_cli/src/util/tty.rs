use std::{
    fmt::Display,
    io::{self, IsTerminal, Read, Write},
};

use anyhow::{Result, bail};
use crossterm::{style::Stylize, terminal};

/// Print `prompt` and read one line. None for a blank line or end of input.
pub fn read_line(prompt: &str) -> Result<Option<String>> {
    write_prompt(prompt)?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(non_blank(&buf))
}

/// Like [`read_line`], but the terminal does not echo the input.
///
/// Errors when the user presses Ctrl-C, since raw mode stops the terminal from
/// sending SIGINT.
pub fn read_hidden(prompt: &str) -> Result<Option<String>> {
    struct RawMode;
    impl Drop for RawMode {
        fn drop(&mut self) {
            let _ = terminal::disable_raw_mode();
        }
    }

    write_prompt(prompt)?;
    let raw_mode = {
        terminal::enable_raw_mode()?;
        RawMode
    };
    // Read through the same buffered stdin as `read_line`, so input typed or
    // pasted ahead of this prompt is not skipped.
    let secret = read_secret(&mut io::stdin().lock())?;
    drop(raw_mode);
    println!();

    let Some(secret) = secret else {
        bail!("Operation aborted.");
    };
    Ok(non_blank(&secret))
}

/// Read raw-mode input up to Enter or end of input. None on Ctrl-C.
///
/// Handles Backspace and Ctrl-U. Drops other control keys and escape
/// sequences, such as arrow keys.
fn read_secret(input: &mut impl Read) -> io::Result<Option<String>> {
    let mut next = || -> io::Result<Option<u8>> {
        let mut byte = [0];
        Ok((input.read(&mut byte)? == 1).then_some(byte[0]))
    };
    let mut secret = Vec::new();
    while let Some(byte) = next()? {
        match byte {
            b'\r' | b'\n' => break,
            0x03 => return Ok(None),
            0x15 => secret.clear(),
            0x08 | 0x7f => {
                // Remove one UTF-8 character: its continuation bytes, then its lead byte.
                while secret.pop().is_some_and(|b| b & 0xc0 == 0x80) {}
            }
            0x1b => {
                if next()? == Some(b'[') {
                    while next()?.is_some_and(|b| !(0x40..=0x7e).contains(&b)) {}
                }
            }
            b if b < 0x20 => {}
            b => secret.push(b),
        }
    }
    Ok(Some(String::from_utf8_lossy(&secret).into_owned()))
}

fn write_prompt(prompt: &str) -> Result<()> {
    let mut stdout = io::stdout();
    write!(stdout, "{prompt}")?;
    Ok(stdout.flush()?)
}

fn non_blank(input: &str) -> Option<String> {
    let input = input.trim();
    (!input.is_empty()).then(|| input.to_string())
}

/// Whether both stdin and stdout are a terminal, so the CLI can prompt.
pub fn is_interactive() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

pub fn stdout_is_tty() -> bool {
    io::stdout().is_terminal()
}

pub fn hyperlink(text: &str, url: &str) -> String {
    format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
}

pub fn link_style(text: &str) -> String {
    text.cyan().underlined().to_string()
}

#[derive(Default)]
pub struct Output {
    lines: Vec<String>,
    tip: Option<String>,
}

impl Output {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line<S: Display>(&mut self, txt: S) -> &mut Self {
        self.lines.push(txt.to_string());
        self
    }

    pub fn tip<S: Into<String>>(&mut self, txt: S) -> &mut Self {
        self.tip.replace(txt.into());
        self
    }

    pub fn print(&self) {
        let out = self.lines.join("\n");

        if let Some(help) = self.tip.as_ref() {
            println!("{out}\n\n{}: {help}", "Tip".bold().underlined());
            return;
        }
        println!("{out}")
    }

    pub fn eprint(&self) {
        let out = self.lines.join("\n");

        if let Some(help) = self.tip.as_ref() {
            eprintln!(
                "{}: {out}\n\n{}: {help}",
                "error".red(),
                "Tip".bold().underlined()
            );
            return;
        }
        eprintln!("{}: {out}", "error".red())
    }
}

#[cfg(test)]
mod tests {
    use super::{hyperlink, read_secret};

    fn secret(input: &[u8]) -> Option<String> {
        read_secret(&mut &input[..]).unwrap()
    }

    #[test]
    fn read_secret_stops_at_enter_and_keeps_the_rest_unread() {
        let mut input = &b"key1\rnext line\n"[..];
        assert_eq!(read_secret(&mut input).unwrap().as_deref(), Some("key1"));
        assert_eq!(input, b"next line\n");
    }

    #[test]
    fn read_secret_handles_editing_keys() {
        assert_eq!(secret(b"abcd\x7f\x7fx\r").as_deref(), Some("abx"));
        assert_eq!(secret(b"wrong\x15right\r").as_deref(), Some("right"));
        assert_eq!(secret(b"ke\x17y\r").as_deref(), Some("key"));
        assert_eq!(secret(b"k\x1b[Dey\r").as_deref(), Some("key"));
        assert_eq!(secret("k\u{e9}\x7fey\r".as_bytes()).as_deref(), Some("key"));
    }

    #[test]
    fn read_secret_aborts_on_ctrl_c() {
        assert_eq!(secret(b"key\x03"), None);
    }

    #[test]
    fn hyperlink_wraps_text_in_an_osc_8_sequence() {
        assert_eq!(
            hyperlink("engine", "https://app.siftstack.com/explore?assets=engine"),
            "\x1b]8;;https://app.siftstack.com/explore?assets=engine\x1b\\engine\x1b]8;;\x1b\\"
        );
    }

    #[test]
    fn hyperlink_leaves_the_visible_width_unchanged() {
        let link = hyperlink("engine", "https://app.siftstack.com");
        assert!(link.contains("engine"));
        assert!(link.starts_with("\x1b]8;;"));
        assert!(link.ends_with("\x1b]8;;\x1b\\"));
    }
}
