use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Display, Formatter};

use unique_pointer::UniquePointer;

use crate::{impl_matcher_for_ref, Match, Matcher, Position, Production, StackRange, State};

pub const ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHA_LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHA_UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHA_NUMERIC: &'static str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub const NUMERIC: &'static str = "0123456789";
pub const ANY: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ascii {
    Alpha,
    AlphaLower,
    AlphaUpper,
    AlphaNumeric,
    Numeric,
    ANY,
}
impl Ascii {
    pub fn collection(&self) -> String {
        match self {
            Ascii::Alpha => ALPHA,
            Ascii::AlphaLower => ALPHA_LOWER,
            Ascii::AlphaUpper => ALPHA_UPPER,
            Ascii::AlphaNumeric => ALPHA_NUMERIC,
            Ascii::Numeric => NUMERIC,
            Ascii::ANY => ANY,
        }
        .to_string()
    }
}
impl_matcher_for_ref!(Ascii);
impl Matcher for Ascii {
    fn as_production(&self) -> Production {
        Production::Ascii(self.clone())
    }

    fn name(&self) -> &'static str {
        match self {
            Ascii::Alpha => "Ascii::Alpha",
            Ascii::AlphaLower => "Ascii::Lower",
            Ascii::AlphaUpper => "Ascii::Upper",
            Ascii::AlphaNumeric => "Ascii::AlphaNumeric",
            Ascii::Numeric => "Ascii::Numeric",
            Ascii::ANY => "Ascii::Numeric",
        }
    }

    fn to_str(&self) -> String {
        self.name().to_string()
    }

    fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        if input.is_empty() {
            None
        } else {
            let target = input[0..1].to_string();
            (target == input && self.collection().contains(&target))
                .then_some((Production::Ascii(self.clone()), start.span_to(&target)).into())
        }
    }
}

impl Display for Ascii {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
