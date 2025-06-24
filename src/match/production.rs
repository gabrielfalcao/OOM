#![allow(unused, static_mut_refs)]
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

use unique_pointer::UniquePointer;

use crate::{
    impl_matcher_for_ref, Ascii, Buffer, Match, Matcher, Position, Span, Special, StackRange, State,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Production {
    And(Vec<Production>),
    Ascii(Ascii),
    Atomic(Box<Production>),
    CompoundAtomic(Box<Production>),
    Literal(String),
    Named(String),
    Not(Box<Production>),
    OneOrMore(Box<Production>),
    Optional(Box<Production>),
    Or(Vec<Production>),
    Range(Range<char>),
    Special(Special),
    ZeroOrMore(Box<Production>),
}
impl Production {
    fn match_literal(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        literal: &str,
    ) -> Option<Match> {
        if input.len() != literal.len() {
            None
        } else {
            (literal == input)
                .then_some((Production::Literal(literal.to_string()), start.span_to(&input)).into())
        }
    }

    fn match_named(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        name: &str,
    ) -> Option<Match> {
        let matcher = state.matcher(name).expect("named matcher");
        if let Some(r#match) = matcher.is_match(state.as_mut(), input, start) {
            Some(
                Into::<Match>::into((self.clone(), self.span(start, input)))
                    .with_inner(vec![r#match]),
            )
        } else {
            None
        }
    }

    fn match_ascii(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        ascii: &Ascii,
    ) -> Option<Match> {
        ascii.is_match(state.as_mut(), input, start)
    }

    fn match_range(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        range: Range<char>,
    ) -> Option<Match> {
        self.match_or(
            state,
            input,
            start,
            &range
                .map(String::from)
                .map(|literal| Production::Literal(literal))
                .collect::<Vec<Production>>(),
        )
    }

    fn match_not(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        if input.is_empty() {
            return Some((self.clone(), self.span(start, input)).into());
        }
        let mut buffer = Buffer::new(input, false);
        if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
            None
        } else {
            Some((self.clone(), self.span(start, input)).into())
        }
    }

    fn match_optional(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        if input.is_empty() {
            return Some((self.clone(), self.span(start, input)).into());
        }
        let mut buffer = Buffer::new(input, false);
        if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
            Some(
                Into::<Match>::into((self.clone(), self.span(start, input)))
                    .with_inner(vec![r#match]),
            )
        } else {
            None
        }
    }

    fn match_special(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Special,
    ) -> Option<Match> {
        if let Some(r#match) = matcher.is_match(state.as_mut(), input, start) {
            Some(
                Into::<Match>::into((self.as_production(), self.span(start, input)))
                    .with_inner(r#match.inner()),
            )
        } else {
            None
        }
    }

    fn match_atomic(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        if (*state).has_epsilon(input) {
            None
        } else {
            let mut buffer = Buffer::new(input, true);
            if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
                Some(
                    Into::<Match>::into((self.clone(), self.span(start, input)))
                        .with_inner(vec![r#match.clone()]),
                )
            } else {
                None
            }
        }
    }

    fn match_compound_atomic(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        if (*state).has_epsilon(input) {
            None
        } else {
            let mut buffer = Buffer::new(input, true);
            if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
                Some(
                    Into::<Match>::into((self.clone(), self.span(start, input)))
                        .with_inner(vec![r#match.clone()]),
                )
            } else {
                None
            }
        }
    }

    fn match_and(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matchers: &Vec<Production>,
    ) -> Option<Match> {
        let mut mindex = 0;
        let maxdex = matchers.len();
        if maxdex == mindex {
            return None;
        }
        let mut buffer = Buffer::new(input, false);
        let mut matches = Vec::new();
        for (index, matcher) in matchers.iter().enumerate() {
            if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
                matches.push(r#match);
            } else {
                return None;
            }
        }
        if matches.is_empty() {
            None
        } else {
            Some(Into::<Match>::into((self.clone(), self.span(start, input))).with_inner(matches))
        }
    }

    fn match_or(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matchers: &Vec<Production>,
    ) -> Option<Match> {
        for matcher in matchers {
            let mut buffer = Buffer::new(input, false);
            if let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
                return Some(r#match.clone());
            }
        }
        None
    }

    fn match_one_or_more(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        let mut buffer = Buffer::new(input, false);
        let mut matches = Vec::new();
        while let Some(r#match) = buffer.produce(state.as_mut(), start, matcher) {
            matches.push(r#match);
        }

        if matches.len() > 0 {
            Some(Into::<Match>::into((self.clone(), start.span_to(input))).with_inner(matches))
        } else {
            None
        }
    }

    fn match_zero_or_more(
        &self,
        state: &mut State,
        input: &str,
        start: &Position,
        matcher: &Production,
    ) -> Option<Match> {
        if input.is_empty() {
            return Some((self.clone(), start.span_to(input)).into());
        }
        self.match_one_or_more(state, input, start, matcher)
    }
}

impl_matcher_for_ref!(Production);
impl Matcher for Production {
    fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        match self {
            Production::And(matchers) => self.match_and(state, input, start, matchers),
            Production::Ascii(ascii) => self.match_ascii(state, input, start, ascii),
            Production::Atomic(matcher) => self.match_atomic(state, input, start, matcher.as_ref()),
            Production::CompoundAtomic(matcher) =>
                self.match_compound_atomic(state, input, start, matcher.as_ref()),
            Production::Literal(literal) => self.match_literal(state, input, start, literal),
            Production::Named(name) => self.match_named(state, input, start, name),
            Production::Not(matcher) => self.match_not(state, input, start, matcher),
            Production::OneOrMore(matcher) =>
                self.match_one_or_more(state, input, start, matcher.as_ref()),
            Production::Optional(matcher) => self.match_optional(state, input, start, matcher),
            Production::Or(matchers) => self.match_or(state, input, start, matchers),
            Production::Range(range) => self.match_range(state, input, start, range.clone()),
            Production::Special(matcher) => self.match_special(state, input, start, &matcher),
            Production::ZeroOrMore(matcher) =>
                self.match_zero_or_more(state, input, start, matcher.as_ref()),
        }
    }

    fn name(&self) -> &str {
        match self {
            Production::And(string) => "Production::And",
            Production::Ascii(ascii) => ascii.name(),
            Production::Atomic(atomic) => "Production::Atomic",
            Production::CompoundAtomic(compound_atomic) => "Production::CompoundAtomic",
            Production::Literal(string) => string.as_str(),
            Production::Named(name) => name.as_str(),
            Production::Not(string) => "Production::Not",
            Production::OneOrMore(string) => "Production::OneOrMore",
            Production::Optional(string) => "Production::Optional",
            Production::Or(string) => "Production::Or",
            Production::Range(range) =>
                format!("Production::Range({}..{})", range.start, range.end).leak(),
            Production::Special(string) => string.name(),
            Production::ZeroOrMore(string) => "Production::ZeroOrMore",
        }
    }

    fn to_str(&self) -> String {
        match self {
            Production::Atomic(string) => {
                format!("Production::Atomic({})", string.to_str())
            },
            Production::CompoundAtomic(string) => {
                format!("Production::CompoundAtomic({})", string.to_str())
            },
            Production::Ascii(string) => {
                format!("Production::Ascii({})", string.to_string())
            },
            Production::Not(matcher) => {
                format!("Production::Not({})", matcher.to_str())
            },
            Production::Optional(matcher) => {
                format!("Production::Optional({})", matcher.to_str())
            },
            Production::Literal(string) => {
                format!("Production::Literal({})", string.to_string())
            },
            Production::Range(range) => {
                format!("Production::Range({}..{})", range.start, range.end)
            },
            Production::Named(string) => {
                format!("Production::Named({})", string.to_string())
            },
            Production::Special(string) => {
                format!("Production::Special({})", string.name())
            },
            Production::And(items) => {
                format!(
                    "Production::And({})",
                    items
                        .iter()
                        .map(|matcher| matcher.to_str())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            Production::Or(items) => {
                format!(
                    "Production::Or({})",
                    items
                        .iter()
                        .map(|matcher| matcher.to_str())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            Production::OneOrMore(matcher) => {
                format!("Production::OneOrMore({})", matcher.as_ref().to_str())
            },
            Production::ZeroOrMore(matcher) => {
                format!("Production::ZeroOrMore({})", matcher.as_ref().to_str())
            },
        }
    }

    fn as_production(&self) -> Production {
        (*self).clone()
    }
}

impl From<&str> for Production {
    fn from(string: &str) -> Production {
        Production::Named(string.to_string())
    }
}
impl From<String> for Production {
    fn from(string: String) -> Production {
        Production::Named(string.clone())
    }
}
impl From<Special> for Production {
    fn from(matcher: Special) -> Production {
        Production::Special(matcher)
    }
}
impl From<Range<char>> for Production {
    fn from(range: Range<char>) -> Production {
        Production::Range(range)
    }
}

impl From<Ascii> for Production {
    fn from(ascii: Ascii) -> Production {
        Production::Ascii(ascii)
    }
}
