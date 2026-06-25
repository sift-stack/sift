use std::io::{self, Write};

use anyhow::Result;
use crossterm::style::Stylize;

#[derive(Default)]
pub struct PromptUser {
    header: Option<String>,
    prompts: Vec<String>,
}

impl PromptUser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header<S: Into<String>>(&mut self, header: S) -> &mut Self {
        self.header.replace(header.into());
        self
    }

    pub fn prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Self {
        self.prompts.push(prompt.into());
        self
    }

    pub fn run(&self) -> Result<Vec<Option<String>>> {
        if self.prompts.is_empty() {
            return Ok(Vec::new());
        }
        let mut user_inputs = Vec::with_capacity(self.prompts.len());

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        if let Some(header) = self.header.as_ref() {
            writeln!(stdout, "{header}")?;
        }

        for prompt in &self.prompts {
            write!(stdout, "{prompt}")?;
            stdout.flush()?;

            let user_input = {
                let mut buf = String::new();
                stdin.read_line(&mut buf)?;
                buf.trim().to_string()
            };
            if user_input.is_empty() {
                user_inputs.push(None);
            } else {
                user_inputs.push(Some(user_input));
            }
        }
        Ok(user_inputs)
    }
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

    pub fn line<S: Into<String>>(&mut self, txt: S) -> &mut Self {
        self.lines.push(txt.into());
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
