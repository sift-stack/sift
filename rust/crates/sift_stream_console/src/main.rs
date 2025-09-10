use clap::Parser;

mod app;
mod cli;
use cli::Command;

fn main() {
    let clargs = cli::Args::parse();
    let terminal = ratatui::init();

    match clargs.command {
        Command::Start { .. } => todo!(),
        Command::Demo => app::demo::run(terminal),
    }
}
