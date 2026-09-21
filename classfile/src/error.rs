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
    /// A bootstrap method argument that is not a loadable constant, carrying the two things the
    /// predicate already knows at the moment it refuses: *which* argument, and what it found there.
    ///
    /// ★ Which variant to use: `InvalidFormat` is for a rule that has nothing to point at, and that
    /// is still most of them. Use this one only when a number is already in hand — folding it into
    /// prose is the thing this variant exists to stop. `UnsupportedVersion` is the same shape and
    /// predates it, so this is the established way here rather than a second scheme.
    ///
    /// It stays `Copy`: two `u16`s and a `&'static str`, no owned data.
    InvalidBootstrapArgument {
        /// Position of the `BootstrapMethods` entry, zero-based, as the attribute stores them.
        method_index: u16,
        /// Position within that entry's argument list, zero-based. OpenJDK calls this
        /// `argument_index` and prints it without the method, which is ambiguous when a class has
        /// more than one bootstrap method; both are carried here for that reason.
        argument_index: u16,
        /// The kind of constant the argument actually names, or `None` when the index names no
        /// pool entry at all — "names nothing" and "names the wrong kind" are different failures
        /// and the message says which.
        actual: Option<&'static str>,
    },
    UnsupportedVersion(u16),
}
