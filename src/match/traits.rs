use std::fmt::{Debug, Display};
use std::ops::{Range, RangeBounds};

use unique_pointer::UniquePointer;

use crate::{Match, Position, Production, Span, State};

pub trait Literal: Sized + Display + Debug + Clone {}
pub trait StackRange: RangeBounds<isize> + Debug + Clone {}

pub trait Matcher: Sized + Debug + Clone {
    fn name(&self) -> &str;
    fn to_str(&self) -> String;
    fn as_production(&self) -> Production;
    fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        None
    }
    fn as_mut(&self) -> &mut Self {
        UniquePointer::read_only(self).extend_lifetime_mut()
    }
    fn to_dbg(&self) -> String {
        format!("{:#?}", self.to_str())
    }
    fn span(&self, start: &Position, input: &str) -> Span {
        start.span_to(input)
    }
}

#[macro_export]
macro_rules! impl_matcher_for_ref {
    ($($type:tt)*) => {
        impl Matcher for &$($type)* {
            fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
                (*self).is_match(state, input, start)
            }

            fn name(&self) -> &str {
                (*self).name()
            }

            fn to_str(&self) -> String {
                (*self).to_str()
            }

            fn as_production(&self) -> Production {
                (*self).as_production()
            }
        }
    };
}
