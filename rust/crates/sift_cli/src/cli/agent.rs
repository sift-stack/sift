use anyhow::Result;
use clap::ValueEnum;
use std::{env::home_dir, path::PathBuf};

/// Agentic skill to install
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Agent {
    /// Anthropic's Claude Code CLI. Writes a SKILL.md file to the user's
    /// Claude skills directory.
    ClaudeCode,

    /// Open AGENTS.md standard, recognized by OpenAI Codex, Cursor, Aider,
    /// Zed, Windsurf, Sourcegraph Amp, Jules, Factory, and RooCode. Writes an
    /// AGENTS.md file at the project root.
    AgentsMd,
}

impl Agent {
    /// Standard on-disk path the agent expects to find skill files at. Some
    /// agents (Claude Code) look in a global per-user directory; others
    /// (AGENTS.md) expect the file at the root of the project the developer
    /// is currently working in.
    pub fn default_skill_path(self) -> Result<PathBuf> {
        match self {
            Self::ClaudeCode => home_dir()
                .map(|p| {
                    p.join(".claude")
                        .join("skills")
                        .join("sift")
                        .join("SKILL.md")
                })
                .ok_or_else(|| anyhow::anyhow!("failed to locate user home directory")),
            Self::AgentsMd => Ok(PathBuf::from("AGENTS.md")),
        }
    }
}
