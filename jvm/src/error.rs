use alloc::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    string::String,
};

use crate::ClassInstance;

#[derive(Debug)]
pub enum JavaError {
    JavaException(Box<dyn ClassInstance>),
    /// A failure that could not be raised as a Java exception, with a message saying what failed.
    ///
    /// A Java exception is an instance of a Java class, so it can only exist once the classes it is
    /// made of are loaded and constructing it has not itself failed. This is what the runtime returns
    /// when that is not the case: `Jvm::new` given a class set missing a class needed before anything
    /// can be raised, or `Jvm::exception` failing again while it is still building an exception on the
    /// same thread. Java code cannot catch it -- there is no instance to catch -- so it propagates to
    /// the host unchanged.
    Unraisable(String),
}

impl Display for JavaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JavaError::JavaException(e) => write!(f, "Java exception: {e:?}"),
            JavaError::Unraisable(message) => write!(f, "unraisable error: {message}"),
        }
    }
}

impl From<JavaError> for anyhow::Error {
    fn from(e: JavaError) -> Self {
        anyhow::anyhow!("{:?}", e)
    }
}
