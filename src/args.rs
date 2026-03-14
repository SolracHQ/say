use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "say")]
#[clap(version = "v0.1.0")]
#[clap(author = "CarlosEduardoL")]
#[clap(
    about = "Rust echo copy",
    long_about = "If -e is in effect, the following sequences are recognized:
  \\\\     backslash
  \\a     alert (BEL)
  \\b     backspace
  \\c     produce no further output
  \\e     escape
  \\f     form feed
  \\n     new line
  \\r     carriage return
  \\t     horizontal tab
  \\v     vertical tab
  \\0NNN  byte with octal value NNN (1 to 3 digits)
  \\xHH   byte with hexadecimal value HH (1 to 2 digits)"
)]
pub struct SayArgs {
    #[clap(value_name = "TEXT")]
    /// Text to be printed on the stdout
    pub text: Vec<String>,
    #[clap(short = 'n')]
    /// do not output the trailing newline
    pub trim_new_line: bool,
    #[clap(short = 'e')]
    /// enable interpretation of backslash escapes
    pub backslash_interpretation: bool,
}
