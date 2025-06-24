#![allow(unused)]
pub(crate) mod errors;

pub use errors::{Error, Result};

pub mod color;
pub use color::{
    ansi, ansi_clear, back, bg, bgfg, couple, fg, fore, from_bytes, from_string, invert_bw, reset,
    rgb_from_bytes, rgb_from_string, wrap,
};

mod macros;

pub mod r#match;
pub use r#match::{Ascii, Buffer, Literal, Match, Matcher, Production, Special, StackRange};

pub mod source;
pub use source::{Position, Source, Span};

pub mod state;
pub use state::State;

pub mod traceback;
pub use traceback::{Caller, Traceback};

pub mod data;
pub use data::Stack;

pub mod token;
pub use token::Token;

pub mod prelude;
