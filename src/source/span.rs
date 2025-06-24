use std::fmt::Display;

use crate::Position;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub input: String,
    pub start: Position,
    pub end: Position,
}
impl Span {
    pub fn new<T: Display>(input: T, start: (usize, usize), end: (usize, usize)) -> Span {
        Span {
            input: input.to_string(),
            start: Position::from_tuple(start),
            end: Position::from_tuple(end),
        }
    }

    pub fn input(&self) -> &str {
        self.input.as_str()
    }

    pub fn with_input(&self, input: &str) -> Span {
        let mut info = self.clone();
        info.input = input.to_string();
        info
    }

    pub fn info(&self) -> Span {
        self.clone()
    }

    pub fn start(&self) -> (usize, usize) {
        self.start.to_tuple()
    }

    pub fn end(&self) -> (usize, usize) {
        self.end.to_tuple()
    }

    pub fn highlight_input(&self, indent: usize) -> String {
        crate::color::fore(self.highlight_input_chars(indent), 32)
    }

    fn highlight_input_chars(&self, indent: usize) -> String {
        let start = self.start.clone();
        let end = self.end.clone();
        self.input
            .lines()
            .enumerate()
            .map(|(no, line)| {
                (
                    no + 1,
                    line.chars()
                        .enumerate()
                        .map(|(no, column)| (no + 1, column.to_string()))
                        .collect::<Vec<(usize, String)>>(),
                )
            })
            .map(|(line, columns)| {
                crate::color::bg(
                    format!(
                        "{}{}",
                        " ".repeat(indent),
                        columns
                            .iter()
                            .map(|(column, text)| {
                                let column = column.clone();
                                if line == start.line && column == start.column {
                                    crate::color::bgfg(text, 235, 198)
                                } else if line == end.line && column == end.column {
                                    [crate::color::reset(""), crate::color::bg(text, 235)].join("")
                                } else {
                                    crate::color::bg(text, 235)
                                }
                            })
                            .collect::<String>()
                    ),
                    235,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.input)
    }
}

impl Into<String> for Span {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<((usize, usize), (usize, usize))> for Span {
    fn into(self) -> ((usize, usize), (usize, usize)) {
        (self.start.to_tuple(), self.end.to_tuple())
    }
}
