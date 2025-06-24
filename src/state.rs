use std::collections::{BTreeMap, VecDeque};
use std::fmt::{Debug, Display};
use std::ops::Range;

use unique_pointer::UniquePointer;

use crate::{
    with_caller, Error, Match, Matcher, Position, Production, Result, Special, Stack, Token,
    Traceback,
};

pub const DEFAULT_WHITESPACE: &'static str = " \t";
pub const DEFAULT_NEWLINE: &'static str = "\n";

#[derive(Debug)]
pub struct State {
    input: Vec<char>,
    in_newline: bool,
    tokens: VecDeque<Token>,
    in_whitespace: bool,
    buffer: String,
    window: String,
    length: usize,
    line: usize,
    column: usize,
    stack: Stack,
    index: usize,
    production_index: usize,
    whitespace: String,
    newline: String,
    productions: BTreeMap<String, Production>,
}
impl Default for State {
    fn default() -> State {
        State {
            input: Vec::new(),
            in_newline: false,
            tokens: VecDeque::new(),
            in_whitespace: false,
            buffer: String::new(),
            window: String::new(),
            length: 0,
            index: 0,
            production_index: 0,
            line: 1,
            column: 1,
            stack: Stack::new(),
            whitespace: DEFAULT_WHITESPACE.to_string(),
            newline: DEFAULT_NEWLINE.to_string(),
            productions: BTreeMap::new(),
        }
    }
}
impl State {
    pub fn with_length(length: usize) -> State {
        let mut state = State::default();
        state.length = length;
        state
    }

    pub fn register_matcher<T: Display>(&mut self, name: T, matcher: Production) -> Result<()> {
        let name = name.to_string();
        if self.productions.contains_key(&name) {
            return Err(with_caller!(Error::new(format!(
                "matcher {:#?} already registered to {:#?}",
                &name, &matcher
            ))));
        }
        self.productions.insert(name, matcher);
        Ok(())
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn stack(&mut self) -> &mut Stack {
        self.stack.as_mut()
    }

    pub fn tokens(&self) -> VecDeque<Token> {
        self.tokens.clone()
    }

    pub fn is_epsilon<T: Display + Debug>(&self, input: T) -> bool {
        self.is_whitespace(input.to_string()) || self.is_newline(input.to_string())
    }

    pub fn has_epsilon<T: Display + Debug>(&self, input: T) -> bool {
        self.has_whitespace(input.to_string()) || self.has_newline(input.to_string())
    }

    pub fn is_whitespace<T: Display + Debug>(&self, input: T) -> bool {
        self.whitespace.to_string() == input.to_string()
            || self.whitespace.contains(&input.to_string())
    }

    pub fn has_whitespace<T: Display + Debug>(&self, input: T) -> bool {
        let input = input.to_string();
        for wc in self.whitespace.chars() {
            if input.contains(wc) {
                return true;
            }
        }
        false
    }

    pub fn is_newline<T: Display + Debug>(&self, input: T) -> bool {
        self.newline.to_string() == input.to_string() || self.newline.contains(&input.to_string())
    }

    pub fn has_newline<T: Display + Debug>(&self, input: T) -> bool {
        let input = input.to_string();
        for wc in self.newline.chars() {
            if input.contains(wc) {
                return true;
            }
        }
        false
    }

    pub fn as_mut<'c>(&self) -> &'c mut State {
        UniquePointer::read_only(self).extend_lifetime_mut()
    }

    pub fn position(&self) -> Position {
        Position::new(self.line, self.column)
    }

    pub fn matcher<T: Display>(&self, name: T) -> Result<Production> {
        let name = name.to_string();
        if let Some(matcher) = self.productions.get(&name) {
            Ok(matcher.clone())
        } else {
            Err(with_caller!(Error::new(format!(
                "no matcher registered with name {:#?}",
                &name
            ))))
        }
    }
}
