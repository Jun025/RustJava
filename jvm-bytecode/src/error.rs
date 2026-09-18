use classfile::ClassFileError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassDefinitionError {
    InvalidClassFile(&'static str),
    UnsupportedClassVersion(u16),
    Verification,
    UnsupportedFeature(&'static str),
}

impl From<ClassFileError> for ClassDefinitionError {
    fn from(error: ClassFileError) -> Self {
        match error {
            ClassFileError::InvalidFormat(cause) => Self::InvalidClassFile(cause),
            ClassFileError::UnsupportedVersion(version) => Self::UnsupportedClassVersion(version),
        }
    }
}
