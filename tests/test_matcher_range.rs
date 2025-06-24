#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_range() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(Production::Range('a'..'z').into());

    let result = first.is_match(state.as_mut(), "string", &state.position());

    assert_equal!(result.clone().unwrap().span().to_string(), "string");
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "s".to_string(),
            "t".to_string(),
            "r".to_string(),
            "i".to_string(),
            "n".to_string(),
            "g".to_string(),
        ])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Literal("s".to_string()),
            Production::Literal("t".to_string()),
            Production::Literal("r".to_string()),
            Production::Literal("i".to_string()),
            Production::Literal("n".to_string()),
            Production::Literal("g".to_string()),
        ])
    );

    Ok(())
}
