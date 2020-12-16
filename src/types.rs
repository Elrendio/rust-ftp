//! The set of valid values for FTP commands

use std::convert::From;
use std::fmt;

/// A shorthand for a Result whose error type is always an FtpError.
pub type Result<T> = ::std::result::Result<T, FtpError>;

/// `FtpError` is a library-global error type to describe the different kinds of
/// errors that might occur while using FTP.
#[derive(Debug)]
pub enum FtpError {
    ConnectionError(::std::io::Error),
    SecureError(String),
    InvalidResponse(String),
    InvalidAddress(::std::net::AddrParseError),
}

/// Text Format Control used in `TYPE` command
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormatControl {
    /// Default text format control (is NonPrint)
    Default,
    /// Non-print (not destined for printing)
    NonPrint,
    /// Telnet format control (\<CR\>, \<FF\>, etc.)
    Telnet,
    /// ASA (Fortran) Carriage Control
    Asa,
}

/// File Type used in `TYPE` command
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    /// ASCII text (the argument is the text format control)
    Ascii(FormatControl),
    /// EBCDIC text (the argument is the text format control)
    Ebcdic(FormatControl),
    /// Image,
    Image,
    /// Binary (the synonym to Image)
    Binary,
    /// Local format (the argument is the number of bits in one byte on local machine)
    Local(u8),
}

impl FormatControl {
    fn as_str(&self) -> &'static str {
        match self {
            &FormatControl::Default | &FormatControl::NonPrint => "N",
            &FormatControl::Telnet => "T",
            &FormatControl::Asa => "C",
        }
    }
}

/// `Line` contains a command code and the contents of a line of text read from the network.
pub struct Line(pub u32, pub String);

impl ToString for FormatControl {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            &FileType::Ascii(ref fc) => format!("A {}", fc.as_str()),
            &FileType::Ebcdic(ref fc) => format!("E {}", fc.as_str()),
            &FileType::Image | &FileType::Binary => String::from("I"),
            &FileType::Local(ref bits) => format!("L {}", bits),
        }
    }
}

impl fmt::Display for FtpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FtpError::ConnectionError(ref ioerr) => write!(f, "FTP ConnectionError: {}", ioerr),
            FtpError::SecureError(ref desc) => write!(f, "FTP SecureError: {}", desc),
            FtpError::InvalidResponse(ref desc) => {
                write!(f, "FTP InvalidResponse: {}", desc)
            }
            FtpError::InvalidAddress(ref perr) => write!(f, "FTP InvalidAddress: {}", perr),
        }
    }
}

impl std::error::Error for FtpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FtpError::ConnectionError(ref ioerr) => Some(ioerr),
            FtpError::SecureError(_) => None,
            FtpError::InvalidResponse(_) => None,
            FtpError::InvalidAddress(ref perr) => Some(perr),
        }
    }
}
