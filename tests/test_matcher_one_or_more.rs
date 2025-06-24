#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_one_or_more_and_literals() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(
        Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );
    let result = first.is_match(state.as_mut(), "O1O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["O1".to_string(), "O1".to_string(),])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())]),
            Production::And(vec![Production::Literal("O".into()), Production::Literal("1".into())]),
        ])
    );
    assert_equal!(result.unwrap().span().to_string(), "O1O1");
    Ok(())
}
#[test]
fn test_one_or_more_or_literals() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );
    let result = first.is_match(state.as_mut(), "O1", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["O".to_string(), "1".to_string(),])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![Production::Literal("O".into()), Production::Literal("1".into())])
    );
    assert_equal!(result.unwrap().span().to_string(), "O1");
    Ok(())
}
#[test]
fn test_one_or_more_literal() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(Production::Literal("O".into()).into());

    let result = first.is_match(state.as_mut(), "OOO", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Literal("O".into()),
            Production::Literal("O".into()),
            Production::Literal("O".into()),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["O".to_string(), "O".to_string(), "O".to_string(),])
    );

    assert_equal!(result.unwrap().span().to_string(), "OOO");
    Ok(())
}

#[test]
fn test_one_or_more_or_literals_repeat_first() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "O1O", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["O".to_string(), "1".to_string(), "O".to_string(),])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Literal("O".into()),
            Production::Literal("1".into()),
            Production::Literal("O".into()),
        ])
    );

    assert_equal!(result.unwrap().span().to_string(), "O1O");
    Ok(())
}

#[test]
fn test_one_or_more_or_literals_empty() -> Result<()> {
    let mut state = State::default();
    let first = Production::OneOrMore(
        Production::Or(vec![Production::Literal("O".into()), Production::Literal("1".into())])
            .into(),
    );

    let result = first.is_match(state.as_mut(), "", &state.position());
    assert_equal!(result, None);
    Ok(())
}
