#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_named_one_or_more_or_literal() -> Result<()> {
    let mut state = State::new("O1");
    state.register_matcher(
        "O1",
        Production::OneOrMore(
            Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
                .into(),
        ),
    );
    let first = Production::Named("O1".to_string());
    let result = first.is_match(state.as_mut(), "O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["O1".to_string()])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![Production::OneOrMore(
            Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
                .into(),
        )])
    );
    assert_equal!(result.unwrap().span().to_string(), "O1");
    Ok(())
}
