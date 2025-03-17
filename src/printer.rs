use {
    crossterm::{
        ExecutableCommand,
        style::{self, Color},
    },
    std::{
        io::{Stderr, Stdout, stderr, stdout},
        mem,
    },
};

#[derive(Debug)]
pub struct StaticStringPrintable<'a> {
    str: &'a str,
    foreground: Option<Color>,
    set_bold: bool,
    reset_color: bool,
}

#[derive(Debug)]
pub struct StringPrintable {
    string: String,
    foreground: Option<Color>,
    set_bold: bool,
    reset_color: bool,
}

#[derive(Debug, Default)]
pub struct Printer<'a> {
    static_strings: &'a [StaticStringPrintable<'a>],
    dynamic_strings: &'a mut [StringPrintable],
    use_stderr: bool,
}

impl<'a> Printer<'a> {
    pub fn print(&mut self) {
        self.static_strings.iter().for_each(|static_string| {
            let outs: (Option<Stdout>, Option<Stderr>) = if !self.use_stderr {
                (Some(stdout()), None)
            } else {
                (None, Some(stderr()))
            };

            if let Some(mut stdout) = outs.0 {
                if let Some(color) = static_string.foreground {
                    stdout.execute(style::SetForegroundColor(color)).unwrap();
                }

                if static_string.set_bold {
                    stdout
                        .execute(style::SetAttribute(style::Attribute::Bold))
                        .unwrap();
                } else {
                    stdout
                        .execute(style::SetAttribute(style::Attribute::NormalIntensity))
                        .unwrap();
                }

                stdout.execute(style::Print(static_string.str)).unwrap();

                if static_string.reset_color {
                    stdout.execute(style::ResetColor).unwrap();
                }
            }

            if let Some(mut stderr) = outs.1 {
                if let Some(color) = static_string.foreground {
                    stderr.execute(style::SetForegroundColor(color)).unwrap();
                }

                stderr.execute(style::Print(static_string.str)).unwrap();

                if static_string.reset_color {
                    stderr.execute(style::ResetColor).unwrap();
                }
            }
        });

        self.dynamic_strings.iter_mut().for_each(|dynamic_string| {
            let outs: (Option<Stdout>, Option<Stderr>) = if !self.use_stderr {
                (Some(stdout()), None)
            } else {
                (None, Some(stderr()))
            };

            if let Some(mut stdout) = outs.0 {
                if let Some(color) = dynamic_string.foreground {
                    stdout.execute(style::SetForegroundColor(color)).unwrap();
                }

                if dynamic_string.set_bold {
                    stdout
                        .execute(style::SetAttribute(style::Attribute::Bold))
                        .unwrap();
                }

                stdout
                    .execute(style::Print(mem::take(&mut dynamic_string.string)))
                    .unwrap();

                if dynamic_string.reset_color {
                    stdout.execute(style::ResetColor).unwrap();
                }
            }

            if let Some(mut stderr) = outs.1 {
                if let Some(color) = dynamic_string.foreground {
                    stderr.execute(style::SetForegroundColor(color)).unwrap();
                }

                stderr
                    .execute(style::Print(mem::take(&mut dynamic_string.string)))
                    .unwrap();

                if dynamic_string.reset_color {
                    stderr.execute(style::ResetColor).unwrap();
                }
            }
        });

        self.static_strings = &[];
        self.dynamic_strings = &mut [];
    }

    #[inline]
    pub fn set_static_strings(&mut self, static_strings: &'a [StaticStringPrintable<'a>]) {
        self.static_strings = static_strings;
    }

    #[inline]
    pub fn set_dynamic_strings(&mut self, dynamic_strings: &'a mut [StringPrintable]) {
        self.dynamic_strings = dynamic_strings;
    }

    #[inline]
    pub fn set_use_stderr(&mut self, use_stderr: bool) {
        self.use_stderr = use_stderr;
    }
}

impl<'a> StaticStringPrintable<'a> {
    pub fn new(str: &'a str, foreground: Option<Color>, set_bold: bool, reset_color: bool) -> Self {
        Self {
            str,
            foreground,
            set_bold,
            reset_color,
        }
    }
}

impl StringPrintable {
    pub fn new(
        string: String,
        foreground: Option<Color>,
        set_bold: bool,
        reset_color: bool,
    ) -> Self {
        Self {
            string,
            foreground,
            set_bold,
            reset_color,
        }
    }
}
