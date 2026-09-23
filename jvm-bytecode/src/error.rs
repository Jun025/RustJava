use alloc::{format, string::String};

use classfile::{ClassFileError, Location};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassDefinitionError {
    InvalidClassFile(&'static str),
    /// The classfile-layer variant of the same name, carried through rather than flattened.
    ///
    /// ★ This layer is `&'static str` like the one below it, so folding this into
    /// `InvalidClassFile` is exactly where the index would be lost — which is the whole point of
    /// the variant existing. `UnsupportedClassVersion` is carried the same way for the same reason.
    InvalidBootstrapArgument {
        method_index: u16,
        argument_index: u16,
        actual: Option<&'static str>,
    },
    /// The classfile-layer `InvalidFormatAt`, carried through for the same reason as the variant above.
    InvalidClassFileAt {
        cause: &'static str,
        location: Location,
    },
    UnsupportedClassVersion(u16),
    Verification,
    UnsupportedFeature(&'static str),
}

impl ClassDefinitionError {
    /// The `ClassFormatError` message for a bootstrap argument that is not a loadable constant.
    ///
    /// ★ It lives here, and not at each boundary, because there are two boundaries — the binary's
    /// runtime and `test-utils` — and a format string copied into both is two sources of truth that
    /// drift. They call this instead.
    ///
    /// Shape: OpenJDK prints `argument_index 4 has bad constant type`, which does not say *which*
    /// bootstrap method when a class has several. This names both.
    pub fn bootstrap_argument_message(method_index: u16, argument_index: u16, actual: Option<&'static str>) -> String {
        match actual {
            Some(kind) => format!("bootstrap method #{method_index} argument #{argument_index} names a {kind}, which is not a loadable constant"),
            None => format!("bootstrap method #{method_index} argument #{argument_index} names no constant pool entry"),
        }
    }
}

impl ClassDefinitionError {
    /// The `ClassFormatError` message for `InvalidClassFileAt` — here for the same two-boundary
    /// reason as `bootstrap_argument_message`. The rule's sentence comes first, unchanged, so a
    /// reader who knew the old message still finds it; the position follows.
    pub fn located_message(cause: &'static str, location: Location) -> String {
        format!("{cause} ({location})")
    }
}

impl From<ClassFileError> for ClassDefinitionError {
    fn from(error: ClassFileError) -> Self {
        match error {
            ClassFileError::InvalidFormat(cause) => Self::InvalidClassFile(cause),
            ClassFileError::InvalidBootstrapArgument {
                method_index,
                argument_index,
                actual,
            } => Self::InvalidBootstrapArgument {
                method_index,
                argument_index,
                actual,
            },
            ClassFileError::InvalidFormatAt { cause, location } => Self::InvalidClassFileAt { cause, location },
            ClassFileError::UnsupportedVersion(version) => Self::UnsupportedClassVersion(version),
        }
    }
}
