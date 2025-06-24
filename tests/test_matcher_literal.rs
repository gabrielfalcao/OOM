#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_literal() -> Result<()> {
    let mut state = State::new("string");
    let first = Production::Literal("string".into());

    let result = first.is_match(state.as_mut(), "string", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "string");
    Ok(())
}

#[test]
fn test_production_or_literals_nonfirst_match() -> Result<()> {
    let mut state = State::new("xyz");
    let first =
        Production::Or(vec![Production::Literal("abc".into()), Production::Literal("xyz".into())]);

    let result = first.is_match(state.as_mut(), "xyz", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(Production::Literal("xyz".into())));
    Ok(())
}

#[test]
fn test_production_or_literals_nonatomic() -> Result<()> {
    let mut state = State::new(" abc ");
    let first =
        Production::Or(vec![Production::Literal("abc".into()), Production::Literal("xyz".into())]);

    let result = first.is_match(state.as_mut(), " abc ", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(Production::Literal("abc".into())));
    Ok(())
}

#[test]
fn test_production_and_literals_non_atomic() -> Result<()> {
    let mut state = State::new("str ing");
    let first =
        Production::And(vec![Production::Literal("str".into()), Production::Literal("ing".into())]);

    let result = first.is_match(state.as_mut(), "str ing", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "str ing");
    Ok(())
}

#[test]
fn test_production_and_literals() -> Result<()> {
    let mut state = State::new("string");
    let first =
        Production::And(vec![Production::Literal("str".into()), Production::Literal("ing".into())]);

    let result = first.is_match(state.as_mut(), "string", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "string");
    Ok(())
}

#[test]
fn test_production_or_literals_first_match() -> Result<()> {
    let mut state = State::new("abc");
    let first =
        Production::Or(vec![Production::Literal("abc".into()), Production::Literal("xyz".into())]);

    let result = first.is_match(state.as_mut(), "abc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(Production::Literal("abc".into())));
    Ok(())
}
