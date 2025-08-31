//! Convert binary data into a human-friendly format.
//!
//! This is the pricklybird command line tool `prbiconv`.
//!
//! # Usage
//! We use `xxd` in these examples to convert raw binary to hexadecimal.
//!
//! By default conversion from pricklybird to bytes is attempted.
//! This can be explicitly set using the `-b` flag.
//! ```console
//! % echo "flea-flux-full" | prbiconv -b | xxd -ps
//! 4243
//! ```
//! 
//! To convert bytes to pricklybird use the `-p` flag.
//! ```console
//! % echo "4243" | xxd -r -p | prbiconv -p
//! flea-flux-full
//! ```

use std::fmt;
use std::io::{self, Read, Write};

use clap::Parser;

use pricklybirdlib::{DecodeError, convert_from_pricklybird, convert_to_pricklybird};

/// The conversion failed.
pub enum AppError {
    /// The conversion failed due to some IO error.
    Io(io::Error),
    /// The conversion failed due to an error while decoding a pricklybird string.
    Decode(DecodeError),
    /// Incorrect arguments were supplied via the CLI.
    ArgumentError(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<DecodeError> for AppError {
    fn from(error: DecodeError) -> Self {
        Self::Decode(error)
    }
}

// Implement Display for AppError to format both error types properly
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Decode(err) => write!(f, "{err}"),
            Self::ArgumentError(msg) => write!(f, "Invalid arguments. {msg}"),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to Display implementation
        fmt::Display::fmt(self, f)
    }
}

#[derive(Parser)]
#[command(
    name = clap::crate_name!(),
    version = clap::crate_version!(),
    about = clap::crate_description!(),
)]
/// Collect arguments supplied via command line.
struct Cli {
    /// Attempt conversion from pricklybird string to bytes.
    #[arg(short = 'b', long = "convert-from-pricklybird")]
    convert_from: bool,

    /// Convert bytes to pricklybird string.
    #[arg(short = 'p', long = "convert-to-pricklybird")]
    convert_to: bool,
}


/// Read from `input` and write to `output`.
/// Attemps conversion from pricklybird string to bytes by default.
/// Setting the `-p` flag will instead convert bytes to a pricklybird string.
fn convert(cli: &Cli, mut input: impl Read, mut output: impl Write) -> Result<(), AppError>{
    if cli.convert_to && cli.convert_from {
        return Err(AppError::ArgumentError(
            "Can not convert from and to pricklybird at the same time.".to_owned(),
        ));
    }
    if cli.convert_to {
        let mut buffer = Vec::<u8>::new();
        let _ = input.read_to_end(&mut buffer)?;
        let output_words = convert_to_pricklybird(&buffer);
        write!(output, "{}", &output_words)?;
        output.flush()?;
    } else {
        let mut buffer = String::new();
        let _ = input.read_to_string(&mut buffer)?;
        let output_bytes = convert_from_pricklybird(&buffer)?;
        output.write_all(&output_bytes)?;
        output.flush()?;
    }
    Ok(())

}

/// Read from stdin and output to stdout.
/// Pass the streams to the `convert` function.
pub fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    convert(&cli, io::stdin(), io::stdout())?;
    Ok(())
}

#[cfg(test)]
mod prbiconv_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_convert_to_pricklybird() {
        let cli = Cli {
            convert_from: false,
            convert_to: true,
        };
        
        let input = Cursor::new([0x42u8, 0x43]);
        let mut output = Cursor::new(Vec::new());
        
        convert(&cli, input, &mut output).unwrap();
        
        let output_words = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!("flea-flux-full", output_words, "prbiconv incorrectly converted 0x4243 to pricklybird string.");
    }

    #[test]
    fn test_convert_from_pricklybird() {
        let cli = Cli {
            convert_from: true,
            convert_to: false,
        };
        
        let input = Cursor::new("flea-flux-full");
        let mut output = Cursor::new(Vec::new());
        
        convert(&cli, input, &mut output).unwrap();
        
        let result_bytes = output.into_inner();
        assert_eq!(vec![0x42u8, 0x43], result_bytes, "prbiconv incorrectly converted 'flea-flux-full' to bytes.");
    }

    #[test]
    fn test_convert_both_flags_error() {
        let cli = Cli {
            convert_from: true,
            convert_to: true,
        };
        
        let input = Cursor::new(Vec::new());
        let mut output = Cursor::new(Vec::new());
        
        let result = convert(&cli, input, &mut output);
        assert!(result.is_err());
        
        assert!(
            matches!(
                result,
                Err(AppError::ArgumentError(_))
            ),
            "prbiconv did not error with both `-p` and `-b` flags set."
        );
    }
}