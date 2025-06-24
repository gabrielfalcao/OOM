use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::Range;

use crate::{
    impl_matcher_for_ref, Buffer, Match, Matcher, Position, Production, StackRange, State,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Special {
    SOI,
    EOI,
    PEEK,                     // PEEK
    PEEK_RANGE(Range<isize>), // PEEK[O..N]
    PEEK_ANY,                 // PEEK[..]
    POP,                      // POP
    DROP,                     // DROP
    PUSH(Box<Production>),    // PUSH()
    WHITESPACE,
}
impl Special {
    pub fn match_soi(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        (state.index() == 0)
            .then_some((Production::Special(Special::SOI.into()), start.span_to(input)).into())
    }

    pub fn match_eoi(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        (state.index() == state.length())
            .then_some((Production::Special(Special::EOI.into()), start.span_to(input)).into())
    }

    pub fn match_peek(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        if let Some(Match { matcher, .. }) = state.stack().peek() {
            if let Some(r#match) = matcher.is_match(state, input, start) {
                return Some(
                    Into::<Match>::into((self.as_production(), self.span(start, input)))
                        .with_inner(vec![r#match.clone()]),
                );
            }
        }
        None
    }

    pub fn match_pop(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        if let Some(m) = self.match_peek(state, input, start) {
            state.stack().pop();
            Some(m)
        } else {
            None
        }
    }

    pub fn match_drop(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        if let Some(Match { matcher, .. }) = state.stack().drop() {
            if let Some(r#match) = matcher.is_match(state, input, start) {
                return Some(
                    Into::<Match>::into((self.as_production(), self.span(start, input)))
                        .with_inner(vec![r#match.clone()]),
                );
            }
        }
        None
    }

    pub fn match_whitespace(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
    ) -> Option<Match> {
        state
            .is_whitespace(input)
            .then_some((self.as_production(), start.span_to(&input)).into())
    }

    pub fn match_push(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        production: &Production,
    ) -> Option<Match> {
        if let Some(r#match) = production.is_match(state, input, start) {
            state.stack().push(r#match.clone());
            let outer = Into::<Match>::into((self.as_production(), self.span(start, "inner")))
                .with_inner(vec![r#match.clone()]);
            Some(outer)
        } else {
            None
        }
    }

    pub fn match_peek_range(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        range: &Range<isize>,
    ) -> Option<Match> {
        let substack = state.stack().peek_range(range)?;
        for r#match in substack {
            let Match { matcher, span, .. } = r#match.clone();
            if span.to_string() == input {
                return Some(r#match.clone());
            }
        }
        None
    }

    pub fn match_peek_any(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
    ) -> Option<Match> {
        let substack = state.stack();
        for r#match in substack {
            let Match { matcher, span, .. } = r#match.clone();
            if span.to_string() == input {
                return Some(r#match.clone());
            }
        }
        None
    }
}
impl_matcher_for_ref!(Special);
impl Matcher for Special {
    fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        match self {
            Special::SOI => self.match_soi(state, input, start),
            Special::EOI => self.match_eoi(state, input, start),
            Special::PEEK => self.match_peek(state, input, start),
            Special::POP => self.match_pop(state, input, start),
            Special::DROP => self.match_drop(state, input, start),
            Special::WHITESPACE => self.match_whitespace(state, input, start),
            Special::PUSH(production) => self.match_push(state, input, start, production),
            Special::PEEK_RANGE(range) => self.match_peek_range(state, input, start, range),
            Special::PEEK_ANY => self.match_peek_any(state, input, start),
        }
    }

    fn name(&self) -> &str {
        match self {
            Special::SOI => "Special::SOI",
            Special::EOI => "Special::EOI",
            Special::PEEK => "Special::PEEK",
            Special::PEEK_ANY => "Special::PEEK_ANY",
            Special::POP => "Special::POP",
            Special::DROP => "Special::DROP",
            Special::WHITESPACE => "Special::WHITESPACE",
            Special::PUSH(_) => "Special::PUSH",
            Special::PEEK_RANGE(_) => "Special::PEEK_RANGE",
        }
    }

    fn as_production(&self) -> Production {
        Production::Special(self.clone())
    }

    fn to_str(&self) -> String {
        match self {
            Special::SOI => {
                format!("Special::SOI")
            },
            Special::EOI => {
                format!("Special::EOI")
            },
            Special::PEEK => {
                format!("Special::PEEK")
            },
            Special::POP => {
                format!("Special::POP")
            },
            Special::DROP => {
                format!("Special::DROP")
            },
            Special::WHITESPACE => {
                format!("Special::WHITESPACE")
            },
            Special::PUSH(r#match) => {
                format!("Special::PUSH({})", r#match.to_str())
            },
            Special::PEEK_RANGE(range) => {
                format!("Special::PEEK_RANGE({:#?})", range)
            },
            Special::PEEK_ANY => {
                format!("Special::PEEK_ANY")
            },
        }
    }
}
