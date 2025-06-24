#![allow(unused)]
use k9::assert_equal;
use oom::{Ascii, Match, Matcher, Production, Result, Special, State};

#[test]
fn test_parse_dummy_data_format() -> Result<()> {
    let mut state = State::new("{one: 'one', foo: 'baz'}");
    state.register_matcher("key", Production::OneOrMore(Production::Ascii(Ascii::Alpha).into()));
    state.register_matcher("open_brace", Production::Literal("{".into()));
    state.register_matcher("colon", Production::Literal(":".into()));
    state.register_matcher("close_brace", Production::Literal("}".into()));
    state.register_matcher("comma", Production::Literal(",".into()));
    state.register_matcher(
        "value",
        Production::And(vec![
            Production::Special(Special::PUSH(Production::Literal("'".into()).into())),
            Production::OneOrMore(
                Production::And(vec![
                    Production::Not(Production::Special(Special::PEEK).into()),
                    Production::Ascii(Ascii::ANY),
                ])
                .into(),
            ),
            Production::Special(Special::POP),
        ]),
    );
    state.register_matcher(
        "item",
        Production::Or(vec![
            Production::And(vec![
                Production::Named("key".to_string()),
                Production::Named("colon".to_string()),
                Production::Named("value".to_string()),
            ]),
            Production::OneOrMore(
                Production::And(vec![
                    Production::Named("key".to_string()),
                    Production::Named("colon".to_string()),
                    Production::Named("value".to_string()),
                    Production::Optional(Production::Named("comma".to_string()).into()),
                ])
                .into(),
            ),
        ]),
    );
    state.register_matcher(
        "items",
        Production::And(vec![
            Production::Named("open_brace".to_string()),
            Production::OneOrMore(Production::Named("item".to_string()).into()),
            Production::Named("close_brace".to_string()),
        ]),
    );

    let first = Production::Named("items".to_string());
    let result = first.is_match(state.as_mut(), "{one: 'one', foo: 'baz'}", &state.position());

    assert_equal!(result.clone().map(|m| m.matcher()), Some(first));
    Ok(())
}
