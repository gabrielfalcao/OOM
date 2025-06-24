#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_atomic_one_or_more_and_literals() -> Result<()> {
    let mut state = State::default();
    let first = Production::Atomic(
        Production::OneOrMore(
            Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())])
                .into(),
        )
        .into(),
    );

    let result = first.is_match(state.as_mut(), "O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.unwrap().span().to_string(), "O1");
    Ok(())
}

#[test]
fn test_atomic_one_or_more_and_literals_mismatch() -> Result<()> {
    let mut state = State::default();
    let first = Production::Atomic(
        Production::OneOrMore(
            Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())])
                .into(),
        )
        .into(),
    );

    let result = first.is_match(state.as_mut(), "O 1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), None);
    Ok(())
}
