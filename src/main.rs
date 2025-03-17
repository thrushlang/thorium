use std::env;

mod cli;
mod commands;
mod logging;
mod parser;
mod printer;

fn main() {
    cli::Cli::parse(env::args().collect());
}
