#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, Clone, Copy)]
pub enum AvailableToolchains {
    LLVM,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolchainSubCommand {
    Install,
    Repair,
    Update,
}

impl ToolchainSubCommand {
    pub fn get_representation() -> [&'static str; 3] {
        ["install", "repair", "update"]
    }
}

impl AvailableToolchains {
    pub fn get_representation() -> [&'static str; 1] {
        ["llvm"]
    }
}

pub fn execute(toolchain: AvailableToolchains, subcommand: ToolchainSubCommand) {
    match subcommand {
        ToolchainSubCommand::Install => install(),
        ToolchainSubCommand::Repair => repair(),
        ToolchainSubCommand::Update => update(),
    }
}

fn update() {}

fn install() {}

fn repair() {}
