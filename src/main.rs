use std::env;

mod cli;
mod commands;
mod logging;
mod parser;

fn main() {
    if cfg!(target_os = "windows") {
        colored::control::set_override(true);
    }

    cli::Cli::parse(env::args().collect());
}
