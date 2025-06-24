// #![allow(unused)]
// use k9::assert_equal;
// use oom::{Ascii, Result, Special, Symbol};

// #[test]
// fn test_atomic_one_or_more() -> Result<()> {
//     let identity = Rule::new(
//         "identity",
//         Rule::from(Sequence::Atomic(Rule::from(Repeat::OOM(
//             Rule::new("identity_string", Rule::from(Ascii::Alpha)).into(),
//         )))),
//     );
//     let string = Rule::new(
//         "string",
//         Sequence::AND(Rule::from(vec![
//             Rule::from(Rule::new("open_double_quotes", Special::PUSH("\"".into()))),
//             Rule::from(Rule::new("not_double_quote", Repeat::NOT("\"".into()))),
//             Rule::from(Rule::new("inner_string", Repeat::ZOM(Ascii::ANY.into()))),
//             Rule::from(Rule::new("close_double_quotes", Special::PEEK)),
//         ])),
//     );
//     let function_call = Rule::new(
//         "function_call",
//         Sequence::AND(Rule::from(vec![
//             Rule::from("identity"),
//             Rule::new(
//                 "function_call_args",
//                 Rule::from(Sequence::OR(Rule::from(vec![
//                     Rule::new(
//                         "function_call_nonspace",
//                         Rule::from(Repeat::ZOM(Ascii::ANY.into())),
//                     ),
//                     Rule::new(
//                         "function_call_space",
//                         Rule::from(Repeat::ZOM(Special::WHITESPACE.into())),
//                     ),
//                 ]))),
//             )
//             .into(),
//         ])),
//     );
//     let item = Rule::new(
//         "item",
//         Rule::from(Sequence::AND(Rule::from(vec![Rule::new(
//             "items",
//             Repeat::OOM(Rule::from(vec![
//                 Symbol::new("function"),
//                 Symbol::new("function_call"),
//             ])),
//         )]))),
//     );
//     let expression =
//         Rule::new("expression", Sequence::AND(Rule::from(vec![Repeat::OOM(item.into())])));
//     let function_block = Rule::new(
//         "function_block",
//         Rule::from(Sequence::AND(Rule::from(vec![
//             Rule::from(Rule::new("open_brace", Rule::from("{"))),
//             Rule::from("expression"),
//             Rule::from(Rule::new("close_brace", Rule::from("}"))),
//         ]))),
//     );
//     let function_args = Rule::new(
//         "function_args",
//         Sequence::AND(Rule::from(vec![
//             Rule::from("("),
//             Rule::from("expression"),
//             Rule::from("}"),
//         ])),
//     );
//     let function = Rule::new(
//         "function",
//         Rule::from(Sequence::AND(Rule::from(vec![
//             Rule::from("function"),
//             identity.into(),
//             function_args.into(),
//             function_block.into(),
//         ]))),
//     );
//     let file = Rule::new(
//         "file",
//         Rule::from(Sequence::AND(Rule::from(vec![
//             Rule::new("SOI", Rule::from(Special::SOI)),
//             expression,
//             Rule::new("EOI", Rule::from(Special::EOI)),
//         ]))),
//     );

//     //     let nodes = file.parse(
//     //         "function hello() {
//     // echo \"hello world\";
//     // }
//     // hello
//     // ",
//     //     )?;

//     //     assert_equal!(nodes.len(), 1);
//     //     assert_equal!(nodes[1].clone(), Node::new("string", "abcxyz", (1, 1), (1, 6)));

//     Ok(())
// }
