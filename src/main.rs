use clap::lazy_static::lazy_static;
use clap::Parser;
use std::process::exit;
use EscapeResult::Inescapable;

use std::io::{self, BufWriter, Stdout, Write};

use crate::args::SayArgs;
use crate::EscapeResult::{Char, Hex, Oct, Return};

mod args;

// Define the command-line arguments using the clap crate
lazy_static! {
    static ref ARGS: SayArgs = SayArgs::parse();
}

fn main() -> io::Result<()> {
    // Combine the text arguments into a single string
    let text = ARGS.text.join(" ");

    // Create a buffered writer to write to stdout
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);

    if ARGS.backslash_interpretation {
        // Interpret escape sequences in the text and write to stdout
        interpretation(text, &mut out)?;
    } else {
        // Write the text as-is to stdout
        out.write_all(text.as_bytes())?;
    }
    // Write a newline character to stdout, depending on whether --trim-new-line was used
    out.write_all(if ARGS.trim_new_line { b"" } else { b"\n" })?;

    // Flush the output buffer to ensure everything is written to stdout
    out.flush()?;
    Ok(())
}

/// Interpret escape sequences in the given text and write the result to the given output stream.
///
/// # Arguments
///
/// * `text` - The text to interpret
/// * `out` - The output stream to write the interpreted text to
///
/// # Errors
///
/// If an error occurs while writing to the output stream, an `io::Error` is returned.
fn interpretation(text: String, out: &mut BufWriter<Stdout>) -> io::Result<()> {
    let mut is_escaped = false;
    let mut skip = 0;
    for (i, char) in text.chars().enumerate() {
        // Skip characters if hex or oct value
        if skip > 0 {
            skip -= 1;
            continue;
        }
        // Check if the current character is escaped
        if !is_escaped {
            match char {
                '\\' => is_escaped = true,
                _ => {
                    out.write(&[char as u8])?;
                }
            }
        } else {
            match escaped(char) {
                Char(c) => {
                    // Write the escaped character to the buffer
                    out.write(&[c as u8])?;
                }
                Return => {
                    // Flush the buffer and exit the program
                    out.flush()?;
                    exit(0)
                }
                Inescapable => {
                    // Write the character as it is with //
                    out.write(&['\\' as u8, char as u8])?;
                }
                Hex => {
                    // Parse the hexadecimal value after '\x' and write to buffer
                    skip = 2;
                    if let Ok(n) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                        out.write(&[(n)])?;
                    } else {
                        // Write '\x' and skip parsing in the next iteration
                        out.write(&['\\' as u8, 'x' as u8])?;
                        skip = 0;
                    }
                }
                Oct => {
                    // Parse the octal value after '\0' and write to buffer
                    skip = 3;
                    if let Ok(n) = u8::from_str_radix(&text[i + 1..i + 4], 8) {
                        out.write(&[n])?;
                    } else {
                        // Write '\0' and skip parsing in the next iteration
                        out.write(&['\\' as u8, '0' as u8])?;
                        skip = 0;
                    }
                }
            }
            // Clear escaped flag after processing the character
            is_escaped = false;
        }
    }
    Ok(())
}

/// Enumerate all possible escape sequences.
enum EscapeResult {
    /// An escape sequence that maps to a single character.
    Char(char),
    /// The escape sequence "\c", which causes the program to exit.
    Return,
    /// An escape sequence that cannot be parsed.
    Inescapable,
    /// An octal escape sequence.
    Oct,
    /// A hexadecimal escape sequence.
    Hex,
}

/// Map an escape sequence to an `EscapeResult`.
fn escaped(c: char) -> EscapeResult {
    match c {
        '\\' => EscapeResult::Char('\\'),
        'a' => EscapeResult::Char('\x07'),
        'b' => EscapeResult::Char('\x08'),
        'e' => EscapeResult::Char('\x1B'),
        'f' => EscapeResult::Char('\x0C'),
        'n' => EscapeResult::Char('\n'),
        'r' => EscapeResult::Char('\r'),
        't' => EscapeResult::Char('\t'),
        'v' => EscapeResult::Char('\x0B'),
        '0' => EscapeResult::Oct,
        'x' => EscapeResult::Hex,
        'c' => EscapeResult::Return,
        _ => EscapeResult::Inescapable,
    }
}
