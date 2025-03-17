use {
    super::{
        logging,
        printer::{Printer, StaticStringPrintable},
    },
    crossterm::style,
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
        match command.trim() {
            "help" | "-h" | "--help" => {
                *index += 1;
                self.help();
            }

            "version" | "-v" | "--version" => {
                *index += 1;
                println!("v{}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }

            "install" => {
                *index += 1;
            }

            "new" => {
                *index += 1;
            }

            "add" => {
                *index += 1;
            }

            "remove" => {
                *index += 1;
            }

            "run" => {
                *index += 1;
            }

            "build" => {
                *index += 1;
            }

            command => {
                *index += 1;
                self.report_error(format!("`{}` is not a command.", command));
            }
        }
    }

    #[inline]
    fn extract_relative_index(&self, index: usize) -> usize {
        if index == self.args.len() {
            index - 1
        } else {
            index
        }
    }

    #[inline]
    fn report_error(&mut self, msg: String) {
        logging::log(logging::LogType::Error, msg);
        process::exit(1);
    }

    fn help(&mut self) {
        let mut printer: Printer = Printer::default();

        let title: &[StaticStringPrintable<'_>; 1] = &[StaticStringPrintable::new(
            "\nThe Thrush Package Manager\n\n",
            Some(style::Color::Rgb {
                r: 141,
                g: 141,
                b: 142,
            }),
            false,
            true,
        )];

        printer.set_static_strings(title);
        printer.print();

        let usage: &[StaticStringPrintable<'_>; 3] = &[
            StaticStringPrintable::new("Usage: ", None, true, true),
            StaticStringPrintable::new(
                "thorium ",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new("[commands]\n\n", None, true, true),
        ];

        printer.set_static_strings(usage);
        printer.print();

        let avaliable_commands: &[StaticStringPrintable<'_>; 1] = &[StaticStringPrintable::new(
            "Avaliable commands:\n\n",
            None,
            true,
            true,
        )];

        printer.set_static_strings(avaliable_commands);
        printer.print();

        let install_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "install",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new("Install compiler dependencies.\n", None, true, true),
        ];

        printer.set_static_strings(install_command);
        printer.print();

        let new_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "new",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new("Create a new project.\n", None, true, true),
        ];

        printer.set_static_strings(new_command);
        printer.print();

        let add_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "add",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new("Add new dependency to the project.\n", None, true, true),
        ];

        printer.set_static_strings(add_command);
        printer.print();

        let remove_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "remove",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new("Remove dependency to the project.\n", None, true, true),
        ];

        printer.set_static_strings(remove_command);
        printer.print();

        let run_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "run",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new(
                "Build and run the entire project into the target.\n",
                None,
                true,
                true,
            ),
        ];

        printer.set_static_strings(run_command);
        printer.print();

        let build_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "build",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new(
                "Build the entire project into the target.\n",
                None,
                true,
                true,
            ),
        ];

        printer.set_static_strings(build_command);
        printer.print();

        let avaliable_commands: &[StaticStringPrintable<'_>; 1] = &[StaticStringPrintable::new(
            "\nUseful commands:\n\n",
            None,
            true,
            true,
        )];

        printer.set_static_strings(avaliable_commands);
        printer.print();

        let version_command: &[StaticStringPrintable<'_>; 5] = &[
            StaticStringPrintable::new("• ", None, true, true),
            StaticStringPrintable::new("(", None, true, true),
            StaticStringPrintable::new(
                "version",
                Some(style::Color::Rgb {
                    r: 141,
                    g: 141,
                    b: 142,
                }),
                true,
                true,
            ),
            StaticStringPrintable::new(") ", None, true, true),
            StaticStringPrintable::new("Show the version.\n", None, true, true),
        ];

        printer.set_static_strings(version_command);
        printer.print();

        process::exit(1);
    }
}
