use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use crate::Span;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub name: String,
    pub span: Span,
    pub inner: VecDeque<Token>,
    pub left: Option<Box<Token>>,
    pub right: Option<Box<Token>>,
}
impl Token {
    pub fn new(name: &str, span: Span) -> Token {
        Token {
            name: name.to_string(),
            span,
            inner: VecDeque::new(),
            left: None,
            right: None,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name.as_str())
    }
}
