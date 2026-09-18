/// Why a class file was refused.
///
/// `InvalidFormat` carries the cause so the rejection can say what is wrong instead of repeating
/// one sentence for every reason — the shape `ClassDefinitionError::UnsupportedFeature` already
/// used. A `&'static str` rather than a variant per rule: the set is open (every new rule adds
/// one) and nothing branches on it, so a string is what a caller actually needs.
///
/// The words are the message a user sees, so they read as a JVM does — "multiple BootstrapMethods
/// attributes", not "AtMostOneBootstrapMethods".
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassFileError {
    InvalidFormat(&'static str),
    UnsupportedVersion(u16),
}
