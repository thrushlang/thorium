use {
    super::logging,
    crate::commands::{
        project,
        toolchain::{self, AvailableToolchains, ToolchainSubCommand},
    },
    colored::Colorize,
    logging::LoggingType,
    std::process,
};

pub struct Cli {
    args: Vec<String>,
}

impl Cli {
    pub fn parse(args: Vec<String>) -> Cli {
        let mut cli: Cli = Self { args };
        cli._parse();

        cli
    }

    fn _parse(&mut self) {
        self.args.remove(0);

        if self.args.is_empty() {
            self.help();
            return;
        }

        let mut depth: usize = 0;

        while depth != self.args.len() {
            self.analyze(&self.args[depth].clone(), &mut depth);
        }
    }

    fn analyze(&mut self, command: &str, index: &mut usize) {
        let command_trimmed: &str = command.trim();

        match command_trimmed {
            "help" | "-h" | "--help" => {
                *index += 1;
                self.help();
            }

            "version" | "-v" | "--version" => {
                *index += 1;
                println!("{}", env!("CARGO_PKG_VERSION"));

                process::exit(0);
            }

            "toolchain" => {
                *index += 1;

                let available_toolchains: [&'static str; 1] =
                    AvailableToolchains::get_representation();

                *index += 1;

                if !available_toolchains.contains(&self.get_arg(index)) {
                    self.help_toolchain();
                }

                match self.get_arg(index) {
                    "llvm" => {
                        *index += 1;

                        let commands: [&'static str; 3] = ToolchainSubCommand::get_representation();

                        if !commands.contains(&self.get_arg(index)) {
                            self.help_toolchain();
                        }

                        match self.get_arg(index) {
                            "install" => {
                                *index += 1;
                                toolchain::execute(
                                    AvailableToolchains::LLVM,
                                    ToolchainSubCommand::Install,
                                );
                            }

                            "repair" => {
                                *index += 1;
                                toolchain::execute(
                                    AvailableToolchains::LLVM,
                                    ToolchainSubCommand::Repair,
                                );
                            }

                            "update" => {
                                *index += 1;
                                toolchain::execute(
                                    AvailableToolchains::LLVM,
                                    ToolchainSubCommand::Update,
                                );
                            }

                            _ => {
                                self.help_toolchain();
                            }
                        }
                    }
                    _ => {
                        self.help_toolchain();
                    }
                }
            }

            "new" => {
                *index += 2;

                let name: &str = self.get_arg(index);

                if name == "new" {
                    self.help_new();
                }

                if name.contains(" ") || name.contains("/") || name.contains("\\") {
                    self.help_new();
                }

                project::create(name);
            }

            "add" => {
                *index += 1;
            }

            "run" => {
                *index += 1;
            }

            "build" => {
                *index += 1;
            }

            _ => {
                *index += 1;
                self.help();
            }
        }
    }

    fn help(&self) {
        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{}\n\n",
                "Thorium Package Manager"
                    .custom_color((141, 141, 142))
                    .bold(),
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {} {}\n\n",
                "Usage:".bold(),
                "thorium".custom_color((141, 141, 142)).bold(),
                "[command]".bold(),
            )
            .as_bytes(),
        );

        logging::write(logging::OutputIn::Stderr, "Commands:\n\n".bold().as_bytes());

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "help".custom_color((141, 141, 142)).bold(),
                "Prints this help message.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "version".custom_color((141, 141, 142)).bold(),
                "Prints the current version.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "toolchain".custom_color((141, 141, 142)).bold(),
                "Manage a toolchain.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "new".custom_color((141, 141, 142)).bold(),
                "Creates a new project.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "add".custom_color((141, 141, 142)).bold(),
                "Adds a dependency.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "run".custom_color((141, 141, 142)).bold(),
                "Runs the project.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "build".custom_color((141, 141, 142)).bold(),
                "Builds the project.".bold()
            )
            .as_bytes(),
        );

        process::exit(1);
    }

    fn help_toolchain(&self) {
        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {} {}\n\n",
                "Thorium Package Manager"
                    .custom_color((141, 141, 142))
                    .bold(),
                "|".bold().bright_white(),
                "Toolchains".custom_color((141, 141, 142)).bold(),
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {} {}\n\n",
                "Usage:".bold(),
                "thorium toolchain".custom_color((141, 141, 142)).bold(),
                "[toolchain] [command]".bold(),
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{}\n {}\n\n",
                "Available toolchains:".bold(),
                format_args!(
                    "{}{}{}{}",
                    "• LLVM Toolchain".custom_color((141, 141, 142)).bold(),
                    " (".bold(),
                    "llvm".bright_white().bold(),
                    ")".bold(),
                )
            )
            .as_bytes(),
        );

        logging::write(logging::OutputIn::Stderr, "Commands:\n\n".bold().as_bytes());

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "install".custom_color((141, 141, 142)).bold(),
                "Installs the specified toolchain.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "repair".custom_color((141, 141, 142)).bold(),
                "Repairs any broken binaries of the toolchain.".bold()
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} [{}] {}\n",
                "•".bold(),
                "update".custom_color((141, 141, 142)).bold(),
                "Updates the toolchain.".bold()
            )
            .as_bytes(),
        );

        process::exit(1);
    }

    fn help_new(&self) {
        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {} {}\n\n",
                "Thorium Package Manager"
                    .custom_color((141, 141, 142))
                    .bold(),
                "|".bold().bright_white(),
                "Create a new project".custom_color((141, 141, 142)).bold(),
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {} {}\n",
                "Usage:".bold(),
                "thorium new".custom_color((141, 141, 142)).bold(),
                "[name]".bold(),
            )
            .as_bytes(),
        );

        logging::write(
            logging::OutputIn::Stderr,
            format!(
                "{} {}\n\n",
                "Note:".bold(),
                "The identifier cannot contain spaces and non UTF-8 characters."
                    .custom_color((141, 141, 142))
                    .bold()
                    .underline(),
            )
            .as_bytes(),
        );

        process::exit(1);
    }

    fn get_arg(&self, index: &usize) -> &str {
        self.args
            .get(*index)
            .unwrap_or(self.args.last().unwrap())
            .trim()
    }

    fn report_error(&mut self, msg: &str) {
        logging::log(LoggingType::Error, msg);
        process::exit(1);
    }
}
