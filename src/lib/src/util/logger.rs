use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/**
 * Prints a nice looking message to console with a consistent style
 * The output will look like this: `[module] message`
 */
pub fn print_debug(module: &str, message: String) {
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
