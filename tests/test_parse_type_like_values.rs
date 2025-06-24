#![allow(unused)]
use k9::assert_equal;
use oom::prelude::*;
use oom::{Matcher, Result, State};

#[test]
fn test_parse_value_unsigned_integer() -> Result<()> {
    let mut state = State::default();
    state.register_matcher(
        "unsigned",
        OneOrMore(
            And(vec![
                Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                Ascii(Numeric.into()).into(),
            ])
            .into(),
        ),
    );

    let first = Named("unsigned".to_string());
    let result = first.is_match(state.as_mut(), "12345", &state.position());

    assert_equal!(result.clone().unwrap().span().to_string(), "12345");
    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
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
        Some(vec![OneOrMore(
            And(vec![
                Or(vec![Literal('0'.to_string()), Range('1'..'9')]),
                Ascii(Numeric.into()).into(),
            ])
            .into(),
        ),])
    );

    Ok(())
}
