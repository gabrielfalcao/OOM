use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

use unique_pointer::UniquePointer;

use crate::{Ascii, Match, Matcher, Position, Production, Span, Special, StackRange, State};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Buffer {
    input: String,
    target: String,
    buffer: String,
    windows: Vec<String>,
    succeeded: Vec<Match>,
    atomic: bool,
}
impl Buffer {
    pub fn new(input: &str, atomic: bool) -> Buffer {
        Buffer {
            input: input.to_string(),
            target: input.to_string(),
            buffer: String::new(),
            windows: Vec::<String>::new(),
            succeeded: Vec::<Match>::new(),
            atomic,
        }
    }

    pub fn produce<T: Matcher>(
        &mut self,
        state: &mut State,
        start: &Position,
        matcher: T,
    ) -> Option<Match> {
        let mut position = start.clone();
        for c in self.target.chars() {
            self.buffer.push(c);
            if let Some(r#match) = matcher.is_match(state.as_mut(), &self.buffer, &position) {
                self.windows.push(self.buffer.clone());
                self.succeeded.push(r#match.clone());
                self.target = self.target.replacen(&self.buffer, "", 1).to_string();
                self.buffer = String::new();
                return Some(r#match);
            } else {
                if !self.atomic && state.is_epsilon(c) {
                    self.buffer.pop();
                }
            }
        }
        None
    }

    pub fn windows(&self) -> Vec<String> {
        self.windows.clone()
    }

    pub fn matches(&self) -> Vec<Match> {
        self.succeeded.clone()
    }
}
