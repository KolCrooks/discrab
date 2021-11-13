use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_with_scope(module: &str, message: String) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    write!(&mut stdout, "[{}]: ", module).unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
    writeln!(&mut stdout, "{}", message).unwrap();
}
