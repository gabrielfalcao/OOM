#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_optional_ascii_alpha() -> Result<()> {
    let mut state = State::default();
    let first = Production::Optional(Production::Ascii(Ascii::Alpha).into());

    let result = first.is_match(state.as_mut(), "O", &state.position());
    assert_equal!(result.clone().unwrap().span().to_string(), "O");

    let result = first.is_match(state.as_mut(), "", &state.position());
    assert_equal!(result.clone().unwrap().span().to_string(), "");
    Ok(())
}
