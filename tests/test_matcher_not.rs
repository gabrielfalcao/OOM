#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_not_ascii_alpha() -> Result<()> {
    let mut state = State::default();
    let first = Production::Not(Production::Ascii(Ascii::Alpha).into());

    let result = first.is_match(state.as_mut(), "N", &state.position());
    assert_equal!(result, None);

    let result = first.is_match(state.as_mut(), "@", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "@");
    Ok(())
}
