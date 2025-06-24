#![allow(unused)]
use k9::assert_equal;
use oom::prelude::*;
use oom::{Matcher, Result, State};

#[test]
fn test_parse_value_unsigned_integer() -> Result<()> {
    let mut state = State::new("12345");
    state.register_matcher(
        "unsigned",
        And(vec![
            Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
            ZeroOrMore(Ascii(Numeric.into()).into()),
        ]),
    );

    let unsigned = Production::Named("unsigned".to_string());

    let result = unsigned.is_match(state.as_mut(), "12345", &state.position());

    assert_equal!(result.clone().unwrap().span().to_string(), "12345");
    assert_equal!(result.clone().map(|m| m.matcher()), Some(unsigned));
    assert_equal!(
        result.clone().map(|m| m
            .inner()
            .iter()
            .map(|m| m.span().to_string())
            .collect::<Vec<String>>()),
        Some(vec!["12345".to_string(),])
    );
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![And(vec![
            Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
            ZeroOrMore(Ascii(Numeric.into()).into()),
        ])])
    );

    Ok(())
}

#[test]
fn test_parse_value_integer() -> Result<()> {
    let mut state = State::new("-12345");
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );
    let signed = Production::Named("signed".to_string());

    let result = signed.is_match(state.as_mut(), "-12345", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(signed.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "-12345");
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            )
        ])])
    );

    let mut state = State::new("12345");
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );
    let result = signed.is_match(state.as_mut(), "12345", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(signed.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "12345");
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(vec![And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ])])
    );

    Ok(())
}

#[test]
fn test_parse_value_float() -> Result<()> {
    let mut state = State::new("-12345.6789");
    state.register_matcher(
        "unsigned",
        And(vec![
            Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
            ZeroOrMore(Ascii(Numeric.into()).into()),
        ]),
    );
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );

    state.register_matcher(
        "float",
        And(vec![
            Unnamed("signed".to_string()),
            Optional(And(vec![Literal(".".to_string()), Unnamed("unsigned".to_string())]).into()),
        ]),
    );
    let float = Production::Named("float".to_string());

    let inner = vec![And(vec![
        Unnamed("signed".to_string()),
        Optional(And(vec![Literal(".".to_string()), Unnamed("unsigned".to_string())]).into()),
    ])];

    let result = float.is_match(state.as_mut(), "-12345.6789", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(float.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "-12345.6789");
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(inner.clone())
    );

    let mut state = State::new("12345");
    state.register_matcher(
        "unsigned",
        And(vec![
            Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
            ZeroOrMore(Ascii(Numeric.into()).into()),
        ]),
    );
    state.register_matcher(
        "signed",
        And(vec![
            Optional(Literal('-'.to_string()).into()),
            OneOrMore(
                And(vec![
                    Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                    Ascii(Numeric.into()).into(),
                ])
                .into(),
            ),
        ]),
    );

    state.register_matcher(
        "float",
        And(vec![
            Unnamed("signed".to_string()),
            Optional(And(vec![Literal(".".to_string()), Unnamed("unsigned".to_string())]).into()),
        ]),
    );
    let result = float.is_match(state.as_mut(), "12345", &state.position());
    assert_equal!(result.clone().map(|m| m.matcher()), Some(float.clone()));
    assert_equal!(result.clone().unwrap().span().to_string(), "12345");
    assert_equal!(
        result
            .clone()
            .map(|m| m.inner().iter().map(|m| m.matcher()).collect::<Vec<Production>>()),
        Some(inner.clone())
    );

    Ok(())
}
