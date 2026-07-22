use core::fmt::{self, Display, Formatter};

/// Reason a class file could not be parsed. Rendered into the
/// `java.lang.ClassFormatError` message, so each variant carries enough
/// context to diagnose the offending file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Truncated,
    BadMagic(u32),
    UnsupportedConstantPoolTag { index: u16, tag: u8 },
    Malformed,
    TrailingData,
}

impl ParseError {
    pub(crate) fn from_nom(err: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => Self::Truncated,
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                if e.code == nom::error::ErrorKind::Eof {
                    Self::Truncated
                } else {
                    Self::Malformed
                }
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "Truncated class file"),
            Self::BadMagic(magic) => write!(f, "Incompatible magic value 0x{magic:08X} in class file"),
            Self::UnsupportedConstantPoolTag { index, tag } => {
                write!(f, "Unknown or unsupported constant pool tag {tag} at index {index} in class file")
            }
            Self::Malformed => write!(f, "Malformed class file"),
            Self::TrailingData => write!(f, "Extra bytes at end of class file"),
        }
    }
}
