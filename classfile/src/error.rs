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
    /// ★ Which variant to use: `InvalidFormat` is for a rule that has nothing to point at;
    /// `InvalidFormatAt` for a rule that stopped at one position in one table; this one for the
    /// bootstrap-argument rule, which holds two indices and what it found. Folding a number already
    /// in hand into prose is the thing these variants exist to stop. `UnsupportedVersion` is the same shape and
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
    /// A rule that walks one of the class file's tables and stopped at a known position.
    ///
    /// ★ One variant for all of them, not one per rule. Eleven of `validate_class`'s rules hold a
    /// position when they refuse, and a variant each is the enum growth the `&'static str` design
    /// was avoiding. What differs between them is only *which table* the number indexes, and there
    /// are five tables — so the table is the enum, and the rule stays the sentence it already was.
    /// `InvalidBootstrapArgument` stays separate because it carries a second index and what it found.
    InvalidFormatAt {
        cause: &'static str,
        location: Location,
    },
    UnsupportedVersion(u16),
}

/// Where in the class file an `InvalidFormatAt` rule stopped.
///
/// The constant pool uses its own 1-based index — the `#N` that `javap -v` prints — because that
/// is how every tool names a pool entry. The others are zero-based positions in their table, the
/// same convention `InvalidBootstrapArgument` uses. Only an index: nothing from the file's bytes
/// (a name, a descriptor) is carried, so a hostile file cannot put text into the message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Location {
    ConstantPoolEntry(u16),
    Interface(u16),
    Field(u16),
    Method(u16),
    ClassAttribute(u16),
    /// A byte offset into the file itself — not a table index, unlike the five above. Only the
    /// "extra bytes" refusal uses it: the offset where the class ends and the extra bytes begin,
    /// which is exact. "truncated or unparsable" was measured and deliberately left without one:
    /// nom's position is where the parser gave up, which for a damaged byte fell more than 8 bytes
    /// away from the damage in roughly half of single-byte mutations — a number that points at the
    /// wrong place half the time is worse than no number.
    ByteOffset(u32),
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ConstantPoolEntry(index) => write!(f, "constant pool entry #{index}"),
            Self::Interface(index) => write!(f, "interface #{index}"),
            Self::Field(index) => write!(f, "field #{index}"),
            Self::Method(index) => write!(f, "method #{index}"),
            Self::ClassAttribute(index) => write!(f, "class attribute #{index}"),
            Self::ByteOffset(offset) => write!(f, "at byte offset {offset}"),
        }
    }
}
