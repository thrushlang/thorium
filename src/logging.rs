use {
    super::printer::{Printer, StringPrintable},
    crossterm::style::{Color, StyledContent, Stylize},
};

#[derive(Debug, PartialEq)]
pub enum LogType {
    Custom(String, Color),
    Warning,
    Panic,
    Error,
}

impl LogType {
    pub fn to_styled(&self) -> StyledContent<&str> {
        match self {
            LogType::Warning => crossterm::style::style("WARNING ").yellow(),
            LogType::Panic => crossterm::style::style("PANIC ").red(),
            LogType::Error => crossterm::style::style("ERROR ").red(),
            LogType::Custom(s, color) => crossterm::style::style(s.as_str()).with(*color),
        }
    }
}

pub fn log(log_type: LogType, message: String) {
    let mut printer: Printer = Printer::default();

    if [LogType::Error, LogType::Panic].contains(&log_type) {
        printer.set_use_stderr(true);
    }

    let strings: &mut [StringPrintable] = &mut [
        StringPrintable::new(log_type.to_styled().to_string(), None, true, true),
        StringPrintable::new(message, None, false, true),
    ];

    printer.set_dynamic_strings(strings);
    printer.print();
}
