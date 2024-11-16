use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    SetPin {
        pin: u8,
        function: String,
        mode: String,
    },

    ClearPin {
        pin: u8,
    },

    Status,
}
