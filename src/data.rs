use std::collections::vec_deque::{IntoIter, Iter};
use std::collections::VecDeque;
use std::convert::{AsMut, AsRef};
use std::iter::{FromIterator, IntoIterator};
use std::ops::Range;

use crate::Match;

#[derive(Debug, Default)]
pub struct Stack {
    items: VecDeque<Match>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack::default()
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn peek(&self) -> Option<Match> {
        if let Some(matcher) = self.items.front() {
            Some(matcher.clone())
        } else {
            None
        }
    }

    pub fn peek_range(&self, range: &Range<isize>) -> Option<Stack> {
        let substack = if range.start >= 0 && range.end >= 0 {
            self.items
                .range(range.start as usize..range.end as usize)
                .map(Clone::clone)
                .collect::<VecDeque<Match>>()
        } else if range.start < 0 && range.end < 0 {
            let mut items = Vec::<Match>::new();
            for item in self.items.clone() {
                items.push(item);
            }
            items[(range.start * -1) as usize..(range.end * -1) as usize]
                .iter()
                .map(Clone::clone)
                .collect::<VecDeque<Match>>()
        } else {
            panic!("invalid range {:#?}", range);
        };
        if substack.is_empty() {
            None
        } else {
            Some(Stack::from_iter(substack))
        }
    }

    pub fn drop(&self) -> Option<Match> {
        if let Some(matcher) = self.items.back() {
            Some(matcher.clone())
        } else {
            None
        }
    }

    pub fn push<T: Into<Match>>(&mut self, matcher: T) -> Option<Match> {
        self.items.push_front(matcher.into());
        if let Some(matcher) = self.items.front() {
            Some(matcher.clone())
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<Match> {
        if let Some(matcher) = self.items.pop_front() {
            Some(matcher)
        } else {
            None
        }
    }
}

impl AsRef<Stack> for Stack {
    fn as_ref(&self) -> &Stack {
        self
    }
}

impl AsMut<Stack> for Stack {
    fn as_mut(&mut self) -> &mut Stack {
        self
    }
}

impl<'c> FromIterator<&'c Match> for Stack {
    fn from_iter<I: IntoIterator<Item = &'c Match>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for r#match in iter {
            stack.push(r#match.clone());
        }
        stack
    }
}

impl FromIterator<Match> for Stack {
    fn from_iter<I: IntoIterator<Item = Match>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for r#match in iter {
            stack.push(r#match);
        }
        stack
    }
}

impl IntoIterator for Stack {
    type IntoIter = IntoIter<Match>;
    type Item = Match;

    fn into_iter(self) -> IntoIter<Match> {
        self.items.into_iter()
    }
}
impl IntoIterator for &mut Stack {
    type IntoIter = IntoIter<Match>;
    type Item = Match;

    fn into_iter(self) -> IntoIter<Match> {
        self.items.clone().into_iter()
    }
}
