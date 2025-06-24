#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_ascii_alpha() -> Result<()> {
    let mut state = State::new("a");
    let first = Production::Ascii(Ascii::Alpha);

    let result = first.is_match(state.as_mut(), "a", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "a");

    let mut state = State::new("G");
    let result = first.is_match(state.as_mut(), "G", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "G");
    Ok(())
}

#[test]
fn test_production_ascii_alpha_lower() -> Result<()> {
    let mut state = State::new("a");
    let first = Production::Ascii(Ascii::AlphaLower);

    let result = first.is_match(state.as_mut(), "a", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "a");
    Ok(())
}

#[test]
fn test_production_ascii_alpha_upper() -> Result<()> {
    let mut state = State::new("G");
    let first = Production::Ascii(Ascii::AlphaUpper);

    let result = first.is_match(state.as_mut(), "G", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "G");
    Ok(())
}

#[test]
fn test_production_ascii_alpha_numeric() -> Result<()> {
    let mut state = State::new("O");
    let first = Production::Ascii(Ascii::AlphaNumeric);

    let result = first.is_match(state.as_mut(), "O", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "O");

    let mut state = State::new("1");
    let result = first.is_match(state.as_mut(), "1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "1");
    Ok(())
}

#[test]
fn test_production_ascii_numeric() -> Result<()> {
    let mut state = State::new("1");
    let first = Production::Ascii(Ascii::Numeric);

    let result = first.is_match(state.as_mut(), "1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "1");
    Ok(())
}

#[test]
fn test_production_ascii_any() -> Result<()> {
    let mut state = State::new("%");
    let first = Production::Ascii(Ascii::ANY);

    let result = first.is_match(state.as_mut(), "%", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first.clone()));
    assert_equal!(result.unwrap().span().to_string(), "%");
    Ok(())
}

#[test]
fn test_one_or_more_ascii() -> Result<()> {
    let mut state = State::new(" O 1 ");
    let first = Production::OneOrMore(Production::Ascii(Ascii::AlphaNumeric).into());

    let result = first.is_match(state.as_mut(), " O 1 ", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), " O 1 ");
    Ok(())
}
