#![allow(unused)]
use k9::assert_equal;
use oom::prelude::*;
use oom::{Matcher, Result, State};

#[test]
fn test_production_optional_ascii_alpha() -> Result<()> {
    let mut state = State::new("O");
    let first = Optional(Alpha.into());

    let result = first.is_match(state.as_mut(), "O", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "O");

    let result = first.is_match(state.as_mut(), "", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "");
    Ok(())
}

#[test]
fn test_production_and_optional_one_or_more() -> Result<()> {
    let mut state = State::new("-12345");
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );

    let first = Named("signed".to_string());

    let result = first.is_match(state.as_mut(), "-12345", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "-12345");

    let mut state = State::new("12345");
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );
    let result = first.is_match(state.as_mut(), "12345", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "12345");

    Ok(())
}
