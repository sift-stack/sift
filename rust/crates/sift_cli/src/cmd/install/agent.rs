use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};

use crate::cli::{AgentSkillsArgs, agent::Agent};

const CLAUDE_CODE_SKILL: &str = include_str!("../../../assets/skills/claude-code/SKILL.md");
const AGENTS_MD_SKILL: &str = include_str!("../../../assets/skills/agents-md/AGENTS.md");

pub fn skills(args: AgentSkillsArgs) -> Result<ExitCode> {
    let AgentSkillsArgs {
        agent,
        output,
        print,
    } = args;

    let content = skill_content(agent);

    if print {
        println!("{content}");
        return Ok(ExitCode::SUCCESS);
    }

    let target_path = match output {
        Some(path) => PathBuf::from(path),
        None => agent.default_skill_path()?,
    };

    if let Some(target_dir) = target_path.parent()
        && !target_dir.as_os_str().is_empty()
        && let Err(err) = fs::create_dir_all(target_dir)
        && err.kind() != ErrorKind::AlreadyExists
    {
        bail!(
            "failed to create directory '{}' for skill file",
            target_dir.display()
        )
    }

    let mut target = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&target_path)
        .with_context(|| format!("failed to open {}", target_path.display()))?;

    write!(target, "{content}")
        .with_context(|| format!("failed to write to file {}", target_path.display()))?;

    println!(
        "Successfully installed {} skill at {}",
        agent_label(agent),
        target_path.display()
    );

    Ok(ExitCode::SUCCESS)
}

fn skill_content(agent: Agent) -> &'static str {
    match agent {
        Agent::ClaudeCode => CLAUDE_CODE_SKILL,
        Agent::AgentsMd => AGENTS_MD_SKILL,
    }
}

fn agent_label(agent: Agent) -> &'static str {
    match agent {
        Agent::ClaudeCode => "Claude Code",
        Agent::AgentsMd => "AGENTS.md",
    }
}
