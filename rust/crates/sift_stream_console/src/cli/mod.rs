use clap::{Parser, Subcommand, crate_description, crate_version};

#[derive(Parser)]
#[command(
    version = crate_version!(),
    about = crate_description!(),
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Starts the sift stream console
    Start {
        /// The address that the sift stream process is listening on
        addr: String,
    },
    /// Starts the demo
    Demo,
}
