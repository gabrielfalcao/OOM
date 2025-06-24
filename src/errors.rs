use std::fmt::{Debug, Display, Formatter};

use crate::{Caller, Traceback};

#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
    callers: Vec<Caller>,
}
impl Error {
    pub fn new<T: Display>(message: T) -> Self {
        let message = message.to_string();
        Error {
            message,
            callers: Vec::new(),
        }
    }
}
impl std::error::Error for Error {}
impl Traceback for Error {
    fn message(&self) -> String {
        self.message.to_string()
    }

    fn with(&self, caller: Caller) -> Self {
        let mut error = self.clone();
        error.callers.insert(0, caller);
        error
    }

    fn callers(&self) -> Vec<Caller> {
        self.callers.to_vec()
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "\x1b[0m{}\n\n\x1b[1;48;5;198m\x1b[1;38;5;235mreason:\x1b[0m {}",
            "TODO",
            self.highlight_message(),
        )
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "\x1b[1;38;5;202min source:\n{}\n\n\x1b[1;38;5;220mStacktrace:\n{}\n",
            self.to_string(),
            self.callers_to_string(4)
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;
