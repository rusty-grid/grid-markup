#![allow(unused)]

use anyhow::Context;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "gm.pest"]
pub struct GridParser;

#[derive(Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Element {
        kind: ElementKind,
        attributes: HashMap<&'a str, &'a str>,
        content: Vec<Node<'a>>,
    },
    RawText(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElementKind {
    Html,
    H1,
    H2,
    H3,
    Div,
    P,
    Br,
    Bold,
    Link,
}

impl<'a> From<&'a str> for ElementKind {
    fn from(value: &'a str) -> Self {
        match value {
            "html" => Self::Html,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "div" => Self::Div,
            "p" => Self::P,
            "br" => Self::Br,
            "b" => Self::Bold,
            "link" => Self::Link,
            v => panic!("invalid node kind {:#?}", v),
        }
    }
}

pub fn parse_str(input: &str) -> anyhow::Result<Node<'_>> {
    let mut pairs = GridParser::parse(Rule::document, input)?;
    // dbg!(&pairs);
    Ok(build_element(pairs.next().context("no root element")?)
        .context("could not build root element")?)
}

fn build_element(pair: pest::iterators::Pair<'_, Rule>) -> Option<Node<'_>> {
    match pair.as_rule() {
        Rule::element => {
            let mut pair = pair.into_inner();
            let kind = ElementKind::try_from(pair.next()?.into_inner().as_str()).ok()?;
            let tail = pair.next()?;
            let attributes = build_attributes(tail.clone());
            if let Some(attributes) = attributes {
                let content = build_elements(pair.next()?.into_inner())?;
                Some(Node::Element {
                    kind,
                    attributes,
                    content,
                })
            } else {
                let content = build_elements(tail.into_inner())?;
                Some(Node::Element {
                    kind,
                    attributes: HashMap::new(),
                    content,
                })
            }
        }
        Rule::content_inner => {
            let contents = pair.as_str();
            if contents.is_empty() {
                None
            } else {
                Some(Node::RawText(contents))
            }
        }
        Rule::document => build_element(pair.into_inner().next()?),
        x => panic!("not an element {:?}", x),
    }
}

fn build_elements(next: pest::iterators::Pairs<'_, Rule>) -> Option<Vec<Node<'_>>> {
    let mut v = Vec::new();
    for pair in next {
        if let Some(e) = build_element(pair) {
            v.push(e);
        }
    }
    Some(v)
}

fn build_attributes(pair: pest::iterators::Pair<'_, Rule>) -> Option<HashMap<&str, &str>> {
    match pair.as_rule() {
        Rule::attribute_map => {
            let mut map = HashMap::new();
            for kv in pair.into_inner() {
                let mut pair = kv.into_inner();
                map.insert(pair.next()?.as_str(), pair.next()?.as_str());
            }
            Some(map)
        }
        x => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_str, ElementKind, Node};

    #[test]
    #[ignore = "not implemented"]
    fn normal_text() {
        let actual = parse_str("normal text\nhello\nmultiline").unwrap();

        let expected = Node::RawText("normal text\nhello\nmultiline");

        assert_eq!(actual, expected);
    }

    #[test]
    fn simple_element() {
        let actual = parse_str("~h1 { Hello There.\nHow are you doing today? }").unwrap();

        let expected = Node::Element {
            kind: ElementKind::H1,
            content: vec![Node::RawText(" Hello There.\nHow are you doing today? ")],
            attributes: Default::default(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn element_with_attributes() {
        let actual = parse_str("~h1(data: test) { with attributes }").unwrap();

        let expected = Node::Element {
            kind: ElementKind::H1,
            content: vec![Node::RawText(" with attributes ")],
            attributes: [("data", "test")].into(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn nested_elements() {
        let actual = parse_str("~div(data: test) {~p{ with attributes }}").unwrap();

        let expected = Node::Element {
            kind: ElementKind::Div,
            content: vec![Node::Element {
                kind: ElementKind::P,
                content: vec![Node::RawText(" with attributes ")],
                attributes: Default::default(),
            }],
            attributes: [("data", "test")].into(),
        };

        assert_eq!(actual, expected);
    }
}
