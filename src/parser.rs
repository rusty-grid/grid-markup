use std::collections::HashMap;

use nom::{
    branch::{alt, permutation, Permutation},
    bytes::complete::{is_a, is_not, tag, take_till, take_until, take_while, take_while1},
    character::complete::{alphanumeric1, anychar, char},
    combinator::{eof, map, map_res, not, opt, peek},
    multi::{many0, many1, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

use crate::node::{Node, NodeKind};

fn whitespace(input: &str) -> IResult<&str, &str> {
    take_while(|c| " \t\r\n".contains(c))(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || "-_./!".contains(c))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '-')(input)
}

fn node_kind(input: &str) -> IResult<&str, NodeKind> {
    map_res(preceded(tag("~"), ident), NodeKind::try_from)(input)
}

fn attribute(input: &str) -> IResult<&str, (&str, &str)> {
    map(
        tuple((
            opt(whitespace),
            word,
            opt(whitespace),
            char(':'),
            opt(whitespace),
            word,
            opt(whitespace),
        )),
        |(_, key, _, _, _, value, _)| (key, value),
    )(input)
}

fn attributes(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    map(
        delimited(char('('), separated_list1(char(','), attribute), char(')')),
        |list| {
            let mut map = HashMap::new();
            for (k, v) in list {
                map.insert(k, v);
            }
            map
        },
    )(input)
}

fn element_node(input: &str) -> IResult<&str, Node> {
    map(
        tuple((
            node_kind,
            opt(whitespace),
            opt(attributes),
            opt(whitespace),
            content,
        )),
        |(kind, _, attributes, _, content)| {
            todo!("construct an element from kind, attributes, and content")
        },
    )(input)
}

fn plaintext_node(input: &str) -> IResult<&str, Node> {
    map(take_while(|c| c != '~'), Node::new_plaintext)(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    alt((element_node, plaintext_node))(input)
}

fn content(input: &str) -> IResult<&str, Vec<Node>> {
    delimited(
        pair(opt(whitespace), char('{')),
        many1(node),
        pair(opt(whitespace), char('}')),
    )(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::map,
        multi::{fold_many0, many0},
        IResult, Parser,
    };

    use crate::{
        node::{Node, NodeKind},
        parser::{attributes, content, node_kind, plaintext_node, element_node},
    };

    #[test]
    fn basic_node_kind() {
        assert_eq!(node_kind("~h1"), Ok(("", NodeKind::Heading1)));
        assert_eq!(node_kind("~p(){}"), Ok(("(){}", NodeKind::Paragraph)));
        assert_eq!(node_kind("~p (){}"), Ok((" (){}", NodeKind::Paragraph)));
    }

    #[test]
    #[should_panic]
    fn invalid_node_kind() {
        assert!(node_kind("~{}").is_ok());
        assert!(node_kind("p{}").is_ok());
        assert!(node_kind("~ p{}").is_ok());
    }

    #[test]
    fn basic_attributes() {
        assert_eq!(attributes("( key: value, bing: bang, boom: true ){}"), {
            let mut map = HashMap::new();
            map.insert("key", "value");
            map.insert("bing", "bang");
            map.insert("boom", "true");
            Ok(("{}", map))
        })
    }

    #[test]
    #[should_panic]
    fn invalid_attributes() {
        assert!(attributes("(bing: bang, boom)").is_ok());
        assert!(attributes("(bing: bang  boom)").is_ok());
        assert!(attributes("(bing, bang: boom)").is_ok());
        assert!(attributes("bing: bang").is_ok());
    }

    #[test]
    fn plaintext() {
        assert_eq!(
            plaintext_node("something ~h1(){}"),
            Ok(("~h1(){}", Node::Plaintext("something")))
        );
        assert_eq!(
            plaintext_node("h1(){} something ~"),
            Ok(("~", Node::Plaintext("h1(){} something")))
        );
        assert_eq!(
            plaintext_node("some other thing"),
            Ok(("", Node::Plaintext("some other thing")))
        );
    }

    #[test]
    fn contents() {
        assert_eq!(
            content("{hello world}"),
            Ok(("", vec![Node::Plaintext("hello world")]))
        );
    }

    #[test]
    fn basic_node() {
        assert_eq!(
            element_node("~h1{}"),
            Ok((
                "",
                Node::new_element(NodeKind::Heading1, HashMap::new(), vec![])
            ))
        );
        assert_eq!(
            element_node("~h1(bing: bang){Hello, world!}"),
            Ok((
                "",
                Node::new_element(
                    NodeKind::Heading1,
                    {
                        let mut map = HashMap::new();
                        map.insert("bing", "bang");
                        map
                    },
                    vec![Node::new_plaintext("Hello, world!")]
                )
            ))
        );
        assert_eq!(
            element_node("~h1{Hello, world!}"),
            Ok((
                "",
                Node::new_element(
                    NodeKind::Heading1,
                    HashMap::new(),
                    vec![Node::new_plaintext("Hello, world!")]
                )
            ))
        )
    }

    #[test]
    fn nested_node() {
        assert_eq!(
            element_node("~div{~p{hello}}"),
            Ok((
                "",
                Node::new_element(
                    NodeKind::Div,
                    HashMap::new(),
                    vec![
                        Node::new_element(
                            NodeKind::Paragraph,
                            HashMap::new(),
                            vec![Node::new_plaintext("hello")]
                        ),
                        Node::new_element(
                            NodeKind::Paragraph,
                            HashMap::new(),
                            vec![Node::new_plaintext("world")]
                        )
                    ]
                )
            ))
        );
    }
}
