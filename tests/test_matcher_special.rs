#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_production_special_push() -> Result<()> {
    let mut state = State::default();
    let first = Production::Special(Special::PUSH(Production::Literal("abc".into()).into()));

    let result = first.is_match(state.as_mut(), "abc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![Production::Literal("abc".into()).into()])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["abc".to_string()])
    );
    assert_equal!(result.unwrap().span().to_string(), "abc");
    Ok(())
}

#[test]
fn test_production_special_pop() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".into()).into())),
        Production::Special(Special::POP),
    ]);

    let result = first.is_match(state.as_mut(), "abcabc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".into()).into())),
            Production::Special(Special::POP),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["abc".to_string(), "abc".to_string(),])
    );
    assert_equal!(result.unwrap().span().to_string(), "abcabc");
    Ok(())
}

#[test]
fn test_production_special_whitespace() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".into()).into())),
        Production::Special(Special::WHITESPACE),
        Production::Special(Special::PEEK),
    ]);

    let result = first.is_match(state.as_mut(), "abc abc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".into()).into())),
            Production::Special(Special::WHITESPACE),
            Production::Special(Special::PEEK),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["abc".to_string(), " ".to_string(), "abc".to_string(),])
    );
    assert_equal!(result.unwrap().span().to_string(), "abc abc");
    Ok(())
}

#[test]
fn test_production_special_peek() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PEEK),
    ]);

    let result = first.is_match(state.as_mut(), "abc,xyz,xyz", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PEEK),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "abc".to_string(),
            ",".to_string(),
            "xyz".to_string(),
            ",".to_string(),
            "xyz".to_string(),
        ])
    );
    assert_equal!(result.unwrap().span().to_string(), "abc,xyz,xyz");
    Ok(())
}

#[test]
fn test_production_special_drop() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::DROP),
    ]);

    let result = first.is_match(state.as_mut(), "abc,xyz,abc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::DROP),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "abc".to_string(),
            ",".to_string(),
            "xyz".to_string(),
            ",".to_string(),
            "abc".to_string(),
        ])
    );
    assert_equal!(result.unwrap().span().to_string(), "abc,xyz,abc");
    Ok(())
}

#[test]
fn test_production_special_peek_range_lower_stack() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PEEK_RANGE(1..2)),
    ]);

    let result = first.is_match(state.as_mut(), "abc,xyz,abc", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PEEK_RANGE(1..2)),
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "abc".to_string(),
            ",".to_string(),
            "xyz".to_string(),
            ",".to_string(),
            "abc".to_string(),
        ])
    );
    assert_equal!(result.unwrap().span().to_string(), "abc,xyz,abc");
    Ok(())
}

#[test]
fn test_production_special_peek_higher_stack() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PEEK_RANGE(0..1)),
    ]);

    let result = first.is_match(state.as_mut(), "abc,xyz,xyz", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.clone().unwrap().span().to_string(), "abc,xyz,xyz");
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "abc".to_string(),
            ",".to_string(),
            "xyz".to_string(),
            ",".to_string(),
            "xyz".to_string(),
        ])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PEEK_RANGE(0..1))
        ])
    );
    Ok(())
}

#[test]
fn test_production_special_peek_any() -> Result<()> {
    let mut state = State::default();
    let first = Production::And(vec![
        Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
        Production::Literal(",".to_string()),
        Production::Special(Special::PEEK_ANY),
    ]);

    let result = first.is_match(state.as_mut(), "abc,xyz,xyz", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    assert_equal!(result.clone().unwrap().span().to_string(), "abc,xyz,xyz");
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![
            Production::Special(Special::PUSH(Production::Literal("abc".to_string()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PUSH(Production::Literal("xyz".into()).into())),
            Production::Literal(",".to_string()),
            Production::Special(Special::PEEK_ANY)
        ])
    );
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec![
            "abc".to_string(),
            ",".to_string(),
            "xyz".to_string(),
            ",".to_string(),
            "xyz".to_string(),
        ])
    );
    Ok(())
}
