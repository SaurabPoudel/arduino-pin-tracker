mod board;
mod cli;
mod pin;

use clap::Parser;
use cli::{Cli, Commands};
use pin::PinMode;

const SAVE_FILE: &str = "aurdino_pins.json";

fn main() {
    let cli = Cli::parse();
    let mut board = board::ArduinoBoard::load_from_file(SAVE_FILE).unwrap_or_else(|e| {
        eprintln!("Error loading board configuration: {}", e);
        std::process::exit(1);
    });

    match cli.command {
        Commands::SetPin {
            pin,
            function,
            mode,
        } => {
            let pin_mode = match mode.to_lowercase().as_str() {
                "input" => PinMode::Input,
                "output" => PinMode::Output,
                "input_pullup" => PinMode::InputPullup,
                "input_pulldown" => PinMode::InputPullup,
                _ => {
                    eprintln!(
                        "Invalid pin mode. Use: input, output, input_pull_up, or input_pulldown"
                    );
                    std::process::exit(1);
                }
            };

            if let Err(e) = board.set_pin_usage(pin, true, Some(function), pin_mode) {
                eprintln!("Error setting pin usage: {}", e);
                std::process::exit(1);
            }
        }

        Commands::ClearPin { pin } => {
            if let Err(e) = board.clear_pin_usage(pin) {
                eprintln!("Error cleaning pin usage: {}", e);
            }
        }

        Commands::Status => {
            board.display_status();
        }
    }

    if let Err(e) = board.save_to_file(SAVE_FILE) {
        eprintln!("Error saving board configuration: {}", e);
        std::process::exit(1);
    }
}
