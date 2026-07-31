mod config;
mod files;
mod skill;

#[cfg(test)]
mod tests;

use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::ExitCode,
};

use anyhow::{Context, Result, anyhow};
use crossterm::style::Stylize;
use semver::Version;

use crate::{
    cli::{AgentInstallArgs, AgentUpdateArgs},
    cmd::version,
    util::progress::Spinner,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum AccessMode {
    ReadOnly,
    Destructive,
}

impl AccessMode {
    fn label(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::Destructive => "destructive tools enabled",
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(super) enum Profile {
    Default,
    Named(String),
}

impl Profile {
    fn from_option(profile: Option<String>) -> Self {
        match profile {
            Some(profile) => Self::Named(profile),
            None => Self::Default,
        }
    }

    fn label(&self) -> String {
        match self {
            Self::Default => "default profile".to_string(),
            Self::Named(profile) => format!("profile '{profile}'"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Registration {
    access: AccessMode,
    profile: Profile,
}

impl Registration {
    fn new(access: AccessMode, profile: Profile) -> Self {
        Self { access, profile }
    }

    fn label(&self) -> String {
        format!("{}, {}", self.access.label(), self.profile.label())
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) enum Harness {
    Claude,
    Codex,
    Cursor,
    OpenCode,
}

impl Harness {
    fn label(self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Codex => "Codex",
            Self::Cursor => "Cursor",
            Self::OpenCode => "OpenCode",
        }
    }
}

pub(super) struct Environment {
    home: PathBuf,
    current_exe: PathBuf,
    path: OsString,
    harnesses: Vec<Harness>,
}

impl Environment {
    fn discover() -> Result<Self> {
        let home = dirs::home_dir().context("failed to locate the user home directory")?;
        let current_exe =
            env::current_exe().context("failed to locate the running sift-cli executable")?;
        let path = env::var_os("PATH").unwrap_or_default();
        let mut environment = Self {
            home,
            current_exe,
            path,
            harnesses: Vec::new(),
        };
        environment.harnesses = environment.detect_harnesses();
        Ok(environment)
    }

    fn detect_harnesses(&self) -> Vec<Harness> {
        let candidates = [
            (Harness::Claude, &["claude"][..], self.home.join(".claude")),
            (Harness::Codex, &["codex"][..], self.home.join(".codex")),
            (
                Harness::Cursor,
                &["cursor", "cursor-agent"][..],
                self.home.join(".cursor"),
            ),
            (
                Harness::OpenCode,
                &["opencode"][..],
                self.home.join(".config").join("opencode"),
            ),
        ];

        candidates
            .into_iter()
            .filter_map(|(harness, commands, config_dir)| {
                let command_exists = commands
                    .iter()
                    .any(|command| self.command_available(command));
                let detected = match harness {
                    Harness::Claude | Harness::Codex => command_exists,
                    Harness::Cursor | Harness::OpenCode => command_exists || config_dir.exists(),
                };
                detected.then_some(harness)
            })
            .collect()
    }

    fn command_available(&self, command: &str) -> bool {
        env::split_paths(&self.path).any(|directory| {
            let direct = directory.join(command);
            if is_executable(&direct) {
                return true;
            }
            cfg!(windows)
                && ["exe", "cmd", "bat"].iter().any(|extension| {
                    is_executable(&directory.join(format!("{command}.{extension}")))
                })
        })
    }

    #[cfg(test)]
    fn for_test(home: PathBuf, current_exe: PathBuf, harnesses: Vec<Harness>) -> Self {
        Self {
            home,
            current_exe,
            path: OsString::new(),
            harnesses,
        }
    }
}

pub fn install(profile: Option<String>, args: AgentInstallArgs) -> Result<ExitCode> {
    let access = if args.allow_destructive {
        AccessMode::Destructive
    } else {
        AccessMode::ReadOnly
    };
    let registration = Registration::new(access, Profile::from_option(profile));
    install_environment(&Environment::discover()?, "Installed", &registration)
}

pub async fn update(profile: Option<String>, args: AgentUpdateArgs) -> Result<ExitCode> {
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;
    match version::fetch_latest().await {
        Ok(Some(latest)) if latest > current => {
            println!("sift-cli {current} is outdated; the current agent bundle was not installed.");
            println!("Update the CLI and its embedded bundle with:");
            println!("\n  {}\n", version::install_command(&latest));
            println!("Then run `sift-cli agent update` again.");
            return Ok(ExitCode::FAILURE);
        }
        Ok(_) => {}
        Err(error) => {
            eprintln!(
                "warning: unable to check GitHub for a newer sift-cli release ({error}); \
                 refreshing the bundle embedded in this CLI"
            );
        }
    }

    let environment = Environment::discover()?;
    let inference = infer_registration(&environment)?;
    let requested_access = if args.allow_destructive {
        Some(AccessMode::Destructive)
    } else if args.read_only {
        Some(AccessMode::ReadOnly)
    } else {
        None
    };
    let requested_profile = if args.default_profile {
        Some(Profile::Default)
    } else {
        profile.map(Profile::Named)
    };

    let unresolved_access =
        requested_access.is_none() && matches!(&inference.access, AccessInference::Mixed);
    let unresolved_profile =
        requested_profile.is_none() && matches!(&inference.profile, ProfileInference::Mixed);
    if unresolved_access || unresolved_profile {
        println!("No changes were made because detected MCP clients are not configured uniformly.");
        if unresolved_access {
            println!("- Access modes are mixed. Choose `--allow-destructive` or `--read-only`.");
        }
        if unresolved_profile {
            println!("- Profiles are mixed. Choose `--profile <name>` or `--default-profile`.");
        }
        return Ok(ExitCode::FAILURE);
    }

    let access = requested_access.unwrap_or_else(|| match inference.access {
        AccessInference::Resolved(access) => access,
        AccessInference::Mixed => unreachable!("mixed access was handled above"),
    });
    let profile = requested_profile.unwrap_or_else(|| match inference.profile {
        ProfileInference::Resolved(profile) => profile,
        ProfileInference::Mixed => unreachable!("mixed profiles were handled above"),
    });
    let registration = Registration::new(access, profile);

    install_environment(&environment, "Updated", &registration)
}

pub async fn doctor(expected_profile: Option<String>) -> Result<ExitCode> {
    let environment = Environment::discover()?;
    println!("Sift agent bundle {}", env!("CARGO_PKG_VERSION"));

    let mut unhealthy = check_release().await;
    let mut blocked = false;
    if environment.harnesses.is_empty() {
        println!("[error] No supported AI coding clients were detected.");
        return Ok(ExitCode::FAILURE);
    }

    println!(
        "Detected: {}",
        environment
            .harnesses
            .iter()
            .map(|harness| harness.label())
            .collect::<Vec<_>>()
            .join(", ")
    );

    for target in skill::targets(&environment) {
        let clients = harness_labels(&target.harnesses);
        match skill::inspect(&target.path)? {
            skill::State::Current => {
                println!("[ok] {clients} skill: {}", target.path.display());
            }
            skill::State::Missing => {
                println!(
                    "[error] {clients} skill is missing: {}",
                    target.path.display()
                );
                unhealthy = true;
            }
            skill::State::ManagedOutdated => {
                println!(
                    "[error] {clients} skill is from a different sift-cli release: {}",
                    target.path.display()
                );
                unhealthy = true;
            }
            skill::State::Conflict => {
                println!(
                    "[error] {clients} has an unmanaged skill at {}",
                    target.path.display()
                );
                unhealthy = true;
                blocked = true;
            }
        }
    }

    // Skill inspection is local file IO, but each Claude/Codex registration costs an `mcp`
    // subprocess round-trip, so inspect every client under a spinner before reporting.
    let spinner = Spinner::new();
    spinner.set_message(format!("{} Sift MCP registrations...", "Checking".green()));
    let mut inspected = Vec::new();
    for harness in &environment.harnesses {
        inspected.push((*harness, config::inspect(*harness, &environment)?));
    }
    spinner.finish_and_clear();

    let mut registrations = Vec::new();
    for (harness, state) in inspected {
        match state {
            config::State::Current(registration) => {
                println!(
                    "[ok] {} MCP registration ({})",
                    harness.label(),
                    registration.label()
                );
                registrations.push(registration);
            }
            config::State::Missing => {
                println!("[error] {} MCP registration is missing", harness.label());
                unhealthy = true;
            }
            config::State::ManagedDrift(registration) => {
                println!(
                    "[error] {} MCP registration differs from the current bundle ({})",
                    harness.label(),
                    registration.label()
                );
                registrations.push(registration);
                unhealthy = true;
            }
            config::State::Conflict(detail) => {
                println!("[error] {} MCP registration: {detail}", harness.label());
                unhealthy = true;
                blocked = true;
            }
            config::State::Unavailable(detail) => {
                println!("[error] {} MCP registration: {detail}", harness.label());
                unhealthy = true;
                blocked = true;
            }
        }
    }
    let access_modes = registrations
        .iter()
        .map(|registration| registration.access)
        .collect::<Vec<_>>();
    let profiles = registrations
        .iter()
        .map(|registration| registration.profile.clone())
        .collect::<Vec<_>>();
    let mixed_access = has_mixed_access_modes(&access_modes);
    if mixed_access {
        println!("[error] Detected MCP clients use mixed access modes.");
        unhealthy = true;
    }
    let mixed_profiles = has_mixed_profiles(&profiles);
    if mixed_profiles {
        println!("[error] Detected MCP clients use mixed profiles.");
        unhealthy = true;
    }
    let expected_profile = expected_profile.map(Profile::Named);
    let unexpected_profile = expected_profile.as_ref().is_some_and(|expected| {
        profiles.len() != environment.harnesses.len()
            || profiles.iter().any(|installed| installed != expected)
    });
    if unexpected_profile {
        println!(
            "[error] Detected MCP clients do not all use the requested {}.",
            expected_profile.as_ref().expect("checked above").label()
        );
        unhealthy = true;
    }

    if unhealthy {
        if blocked {
            println!(
                "Resolve the reported conflicts before repairing all detected integrations \
                 together."
            );
        }
        if mixed_access {
            println!(
                "Choose one explicitly with `sift-cli agent update --allow-destructive` or \
                 `sift-cli agent update --read-only`."
            );
        }
        if mixed_profiles {
            match expected_profile.as_ref() {
                Some(Profile::Named(profile)) => {
                    println!(
                        "Run `sift-cli agent update --profile {profile}` to switch every detected \
                         integration."
                    );
                }
                _ => {
                    println!(
                        "Choose one explicitly with `sift-cli agent update --profile <name>` or \
                         `sift-cli agent update --default-profile`."
                    );
                }
            }
        }
        if unexpected_profile && !mixed_profiles {
            println!(
                "Run `sift-cli agent update --profile {}` to switch every detected integration.",
                match expected_profile.as_ref().expect("checked above") {
                    Profile::Named(profile) => profile,
                    Profile::Default => unreachable!("doctor only accepts named profiles"),
                }
            );
        }
        if blocked && !mixed_access && !mixed_profiles && !unexpected_profile {
            println!(
                "Then run `sift-cli agent update` to repair all detected integrations together."
            );
        }
        if !blocked && !mixed_access && !mixed_profiles && !unexpected_profile {
            println!("Run `sift-cli agent update` to repair the detected integrations.");
        }
        Ok(ExitCode::FAILURE)
    } else {
        println!("All detected Sift agent integrations are healthy.");
        Ok(ExitCode::SUCCESS)
    }
}

pub fn uninstall() -> Result<ExitCode> {
    uninstall_environment(&Environment::discover()?)
}

fn uninstall_environment(environment: &Environment) -> Result<ExitCode> {
    if environment.harnesses.is_empty() {
        println!("No supported AI coding clients were detected; nothing to uninstall.");
        return Ok(ExitCode::SUCCESS);
    }

    let targets = skill::targets(environment);
    let mut blockers = Vec::new();
    for target in &targets {
        if skill::inspect(&target.path)? == skill::State::Conflict {
            blockers.push(format!(
                "{} has an unmanaged skill at {}",
                harness_labels(&target.harnesses),
                target.path.display()
            ));
        }
    }
    for harness in &environment.harnesses {
        match config::inspect(*harness, environment)? {
            config::State::Conflict(detail) | config::State::Unavailable(detail) => {
                blockers.push(format!("{} MCP registration: {detail}", harness.label()));
            }
            _ => {}
        }
    }
    if !blockers.is_empty() {
        println!("No changes were made because the existing setup needs attention:");
        for blocker in blockers {
            println!("  - {blocker}");
        }
        return Ok(ExitCode::FAILURE);
    }

    for target in targets {
        let clients = harness_labels(&target.harnesses);
        match skill::inspect(&target.path)? {
            skill::State::Missing => println!("[skip] {clients} skill was not installed"),
            skill::State::Current | skill::State::ManagedOutdated => {
                skill::uninstall(&target.path)?;
                println!("[removed] {clients} skill: {}", target.path.display());
            }
            skill::State::Conflict => {
                unreachable!("unmanaged skills are rejected during preflight");
            }
        }
    }

    for harness in &environment.harnesses {
        let state = config::inspect(*harness, environment)?;
        match &state {
            config::State::Missing => {
                println!(
                    "[skip] {} MCP registration was not installed",
                    harness.label()
                );
            }
            config::State::Conflict(detail) | config::State::Unavailable(detail) => {
                unreachable!(
                    "{} config changed after preflight: {detail}",
                    harness.label()
                );
            }
            config::State::Current(_) | config::State::ManagedDrift(_) => {
                config::uninstall(*harness, environment)?;
                println!("[removed] {} MCP registration", harness.label());
            }
        }
    }

    println!("Removed the Sift agent bundle from every detected client.");
    Ok(ExitCode::SUCCESS)
}

fn install_environment(
    environment: &Environment,
    verb: &str,
    registration: &Registration,
) -> Result<ExitCode> {
    if environment.harnesses.is_empty() {
        println!(
            "No supported AI coding clients were detected. Supported clients: \
             Claude Code, Codex, Cursor, and OpenCode."
        );
        return Ok(ExitCode::FAILURE);
    }

    // Each detected Claude/Codex client costs several `mcp` subprocess round-trips, so
    // hold a spinner across the whole silent stretch that precedes the result lines.
    let spinner = Spinner::new();
    spinner.set_message(format!("{} Sift MCP and skills...", "Installing".green()));

    let targets = skill::targets(environment);
    let mut blockers = Vec::new();
    for target in &targets {
        if skill::inspect(&target.path)? == skill::State::Conflict {
            blockers.push(format!(
                "{} has an unmanaged skill at {}",
                harness_labels(&target.harnesses),
                target.path.display()
            ));
        }
    }
    for harness in &environment.harnesses {
        let state = config::inspect(*harness, environment)?;
        match state {
            config::State::Conflict(detail) | config::State::Unavailable(detail) => {
                blockers.push(format!("{} MCP registration: {detail}", harness.label()));
            }
            _ => {}
        }
    }

    if !blockers.is_empty() {
        spinner.finish_and_clear();
        println!("No changes were made because the existing setup needs attention:");
        for blocker in blockers {
            println!("  - {blocker}");
        }
        return Ok(ExitCode::FAILURE);
    }

    let config_snapshots = environment
        .harnesses
        .iter()
        .map(|harness| config::snapshot(*harness, environment))
        .collect::<Result<Vec<_>>>()?;
    let mut replacements = Vec::new();
    let mut installed_configs = 0;
    let apply_result = (|| -> Result<()> {
        for target in &targets {
            replacements.push(skill::begin_install(&target.path)?);
        }
        for harness in &environment.harnesses {
            config::install(*harness, environment, registration)?;
            installed_configs += 1;
        }
        Ok(())
    })();
    if let Err(error) = apply_result {
        let mut rollback_errors = Vec::new();
        for snapshot in config_snapshots[..installed_configs].iter().rev() {
            if let Err(rollback_error) = config::restore(snapshot, environment) {
                rollback_errors.push(rollback_error.to_string());
            }
        }
        for replacement in replacements.into_iter().rev() {
            if let Err(rollback_error) = replacement.rollback() {
                rollback_errors.push(rollback_error.to_string());
            }
        }
        if rollback_errors.is_empty() {
            return Err(error.context("all earlier agent integration changes were rolled back"));
        }
        return Err(anyhow!(
            "{error:#}; rollback also failed: {}",
            rollback_errors.join("; ")
        ));
    }

    let mut cleanup_errors = Vec::new();
    for replacement in replacements {
        if let Err(error) = replacement.commit() {
            cleanup_errors.push(error.to_string());
        }
    }
    if !cleanup_errors.is_empty() {
        return Err(anyhow!(
            "installed the agent bundle but failed to remove temporary backups: {}",
            cleanup_errors.join("; ")
        ));
    }

    spinner.finish_and_clear();

    for target in &targets {
        println!(
            "[ok] {verb} {} skill: {}",
            harness_labels(&target.harnesses),
            target.path.display()
        );
    }
    for harness in &environment.harnesses {
        println!(
            "[ok] {verb} {} MCP registration ({})",
            harness.label(),
            registration.label()
        );
    }

    println!(
        "{verb} the Sift agent bundle for: {} ({}).",
        harness_labels(&environment.harnesses),
        registration.label()
    );
    Ok(ExitCode::SUCCESS)
}

#[derive(Debug, Eq, PartialEq)]
enum AccessInference {
    Resolved(AccessMode),
    Mixed,
}

#[derive(Debug, Eq, PartialEq)]
enum ProfileInference {
    Resolved(Profile),
    Mixed,
}

struct RegistrationInference {
    access: AccessInference,
    profile: ProfileInference,
}

fn infer_registration(environment: &Environment) -> Result<RegistrationInference> {
    let mut registrations = Vec::new();
    for harness in &environment.harnesses {
        match config::inspect(*harness, environment)? {
            config::State::Current(registration) | config::State::ManagedDrift(registration) => {
                registrations.push(registration);
            }
            _ => {}
        }
    }
    let access_modes = registrations
        .iter()
        .map(|registration| registration.access)
        .collect::<Vec<_>>();
    let profiles = registrations
        .into_iter()
        .map(|registration| registration.profile)
        .collect::<Vec<_>>();
    Ok(RegistrationInference {
        access: infer_access_modes(&access_modes),
        profile: infer_profiles(&profiles),
    })
}

fn infer_access_modes(access_modes: &[AccessMode]) -> AccessInference {
    if has_mixed_access_modes(access_modes) {
        AccessInference::Mixed
    } else {
        AccessInference::Resolved(
            access_modes
                .first()
                .copied()
                .unwrap_or(AccessMode::ReadOnly),
        )
    }
}

fn has_mixed_access_modes(access_modes: &[AccessMode]) -> bool {
    access_modes
        .first()
        .is_some_and(|first| access_modes.iter().any(|access| access != first))
}

fn infer_profiles(profiles: &[Profile]) -> ProfileInference {
    if has_mixed_profiles(profiles) {
        ProfileInference::Mixed
    } else {
        ProfileInference::Resolved(profiles.first().cloned().unwrap_or(Profile::Default))
    }
}

fn has_mixed_profiles(profiles: &[Profile]) -> bool {
    profiles
        .first()
        .is_some_and(|first| profiles.iter().any(|profile| profile != first))
}

async fn check_release() -> bool {
    let current = match Version::parse(env!("CARGO_PKG_VERSION")) {
        Ok(current) => current,
        Err(error) => {
            println!("[warning] Could not parse the installed CLI version: {error}");
            return false;
        }
    };
    let latest = {
        let spinner = Spinner::new();
        spinner.set_message(format!(
            "{} for a newer sift-cli release...",
            "Checking".green()
        ));
        version::fetch_latest().await
    };
    match latest {
        Ok(Some(latest)) if latest > current => {
            println!("[error] sift-cli {current} is outdated; latest is {latest}");
            println!("Update with:\n\n  {}\n", version::install_command(&latest));
            true
        }
        Ok(Some(_)) => {
            println!("[ok] sift-cli {current} is current");
            false
        }
        Ok(None) => {
            println!("[warning] No stable sift-cli releases were found on GitHub");
            false
        }
        Err(error) => {
            println!("[warning] Could not check for a newer sift-cli release: {error}");
            false
        }
    }
}

fn harness_labels(harnesses: &[Harness]) -> String {
    harnesses
        .iter()
        .map(|harness| harness.label())
        .collect::<Vec<_>>()
        .join("/")
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}
