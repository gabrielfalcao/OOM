#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_zero_or_more_literal() -> Result<()> {
    let mut state = State::new("OOO");
    let first = Production::ZeroOrMore(Production::Literal("O".into()).into());

    let result = first.is_match(state.as_mut(), "OOO", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "OOO");
    Ok(())
}

#[test]
fn test_zero_or_more_and_literals() -> Result<()> {
    let mut state = State::new("O1");
    let first = Production::ZeroOrMore(
        Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "O1");
    Ok(())
}

#[test]
fn test_zero_or_more_or_literals() -> Result<()> {
    let mut state = State::new("O1");
    let first = Production::ZeroOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "O1");
    Ok(())
}
#[test]
fn test_zero_or_more_or_literals_repeat_first() -> Result<()> {
    let mut state = State::new("O1O");
    let first = Production::ZeroOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "O1O", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "O1O");
    Ok(())
}

#[test]
fn test_zero_or_more_empty() -> Result<()> {
    let mut state = State::new("");
    let first = Production::ZeroOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "");
    Ok(())
}
