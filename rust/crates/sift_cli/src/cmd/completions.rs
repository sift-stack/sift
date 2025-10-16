use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::ExitCode,
    str::FromStr,
};

use anyhow::{Context, Result, anyhow};
use clap::{CommandFactory, crate_name};
use clap_complete::{Shell, generate};
use crossterm::style::Stylize;

use crate::{
    cli::{self, CompletionsPrintArgs},
    util::tty::Output,
};

pub fn print(args: CompletionsPrintArgs) -> Result<ExitCode> {
    let shell = match args.shell {
        Some(sh) => sh,
        None => try_get_shell()?,
    };

    let mut cmd = cli::Args::command();
    let mut stdout = io::stdout();

    generate(shell, &mut cmd, crate_name!(), &mut stdout);

    Ok(ExitCode::SUCCESS)
}

pub fn update() -> Result<ExitCode> {
    let shell = try_get_shell()?;
    let root = get_config_root()?;

    let (dir_path, filename) = match try_get_shell()? {
        Shell::Zsh => (root.join(".zsh-complete"), format!("_{}", crate_name!())),
        Shell::Bash => (root.join(".bash_completion.d"), String::from(crate_name!())),
        Shell::Fish => (
            get_fish_completions_dir(&root),
            format!("{}.fish", crate_name!()),
        ),
        _ => {
            Output::new()
                .line(format!(
                    "failed to automatically update completions due to unsupported shell {shell}"
                ))
                .tip(format!(
                    "try manually generating completions with `{}`",
                    "completions print".cyan()
                ))
                .eprint();
            return Ok(ExitCode::FAILURE);
        }
    };

    fs::create_dir_all(&dir_path)?;
    let completions_file_path = dir_path.join(filename);

    let mut completions_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&completions_file_path)
        .with_context(|| format!("failed to create/open {}", completions_file_path.display()))?;

    let mut cmd = cli::Args::command();
    generate(shell, &mut cmd, crate_name!(), &mut completions_file);

    let mut out = Output::new();
    out.line(format!(
        "{} {}",
        "Updated".green(),
        completions_file_path.display()
    ));

    match shell {
        Shell::Zsh => {
            out.tip(format!(
                "Ensure \"{}\" is set in your {} and restart your shell (sourcing doesn't always work)",
                "fpath=($HOME/.zsh-complete $fpath)".cyan(),
                "$HOME/.zshrc".cyan(),
            ));
        }
        Shell::Bash => {
            out.tip("Don't forget to restart your shell (sourcing doesn't always work)");
        }
        Shell::Fish => {
            out.tip(format!(
                "Fish will automatically load completions from {}. \
                If you don’t see them right away, restart your shell with `{}`.",
                "~/.config/fish/completions".cyan(),
                "exec fish".yellow(),
            ));
        }
        _ => (),
    }
    out.print();

    Ok(ExitCode::SUCCESS)
}

fn try_get_shell() -> Result<Shell> {
    env::var_os("SHELL")
        .map(PathBuf::from)
        .and_then(|path| {
            path.as_path()
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| Shell::from_str(n).ok())
        })
        .ok_or(anyhow!(
            "failed to infer user shell from \"$SHELL\" environment variable"
        ))
}

#[cfg(target_os = "macos")]
pub fn get_fish_completions_dir(root: &Path) -> PathBuf {
    root.join(".config").join("fish").join("completions")
}

#[cfg(target_os = "linux")]
pub fn get_fish_completions_dir(root: &Path) -> PathBuf {
    root.join("fish").join("completions")
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn get_fish_completions_dir(root: &Path) -> PathBuf {
    unreachable!("automatic updating of completions is only supported on macos and linux")
}

/// Home directory for macos.
#[cfg(target_os = "macos")]
pub fn get_config_root() -> Result<PathBuf> {
    env::home_dir().ok_or(anyhow!("unable to determine home directory"))
}

/// `XDG_CONFIG_HOME` or `HOME` for linux.
#[cfg(target_os = "linux")]
pub fn get_config_root() -> Result<PathBuf> {
    dirs::config_local_dir()
        .or_else(env::home_dir)
        .ok_or(anyhow!(
            "unable to determine config directory from \"$XDG_CONFIG_HOME\" or \"$HOME\""
        ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn get_config_root() -> Result<PathBuf> {
    Err(anyhow!(
        "automatic updating of completions is only supported on macos and linux"
    ))
}
