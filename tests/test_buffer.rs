#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Buffer, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_buffer_contiguous() -> Result<()> {
    let mut state = State::new("abc,xyz");
    let abc = Production::Literal("abc".into());
    let comma = Production::Literal(",".into());
    let xyz = Production::Literal("xyz".into());

    let mut buffer = Buffer::new("abc,xyz", true);

    let mut matches = Vec::<Option<Match>>::new();

    let start = state.position();
    matches.push(buffer.produce(state.as_mut(), &start, &abc));
    matches.push(buffer.produce(state.as_mut(), &start, &comma));
    matches.push(buffer.produce(state.as_mut(), &start, &xyz));

    assert_equal!(
        matches
            .iter()
            .map(|m| m.clone().map(|m| m.span().to_string()).unwrap_or_default())
            .collect::<Vec<String>>(),
        vec!["abc".to_string(), ",".to_string(), "xyz".to_string(),]
    );

    Ok(())
}

#[test]
fn test_buffer_non_atomic() -> Result<()> {
    let mut state = State::new("abc,xyz");
    let abc = Production::Literal("abc".into());
    let comma = Production::Literal(",".into());
    let xyz = Production::Literal("xyz".into());

    let mut buffer = Buffer::new("abc , xyz", false);

    let mut matches = Vec::<Option<Match>>::new();

    let start = state.position();
    matches.push(buffer.produce(state.as_mut(), &start, &abc));
    matches.push(buffer.produce(state.as_mut(), &start, &comma));
    matches.push(buffer.produce(state.as_mut(), &start, &xyz));

    assert_equal!(
        matches
            .iter()
            .map(|m| m.clone().map(|m| m.span().to_string()).unwrap_or_default())
            .collect::<Vec<String>>(),
        vec!["abc".to_string(), ",".to_string(), "xyz".to_string(),]
    );

    Ok(())
}
