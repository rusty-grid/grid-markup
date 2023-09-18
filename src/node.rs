use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Plaintext(&'a str),
    Element {
        kind: NodeKind,
        attributes: HashMap<&'a str, &'a str>,
        content: Vec<Self>,
    },
}

impl<'a> Node<'a> {
    pub fn new_plaintext(s: &'a str) -> Self {
        Self::Plaintext(s.trim())
    }
    pub fn new_element(
        kind: NodeKind,
        attributes: HashMap<&'a str, &'a str>,
        content: Vec<Self>,
    ) -> Self {
        Self::Element {
            kind,
            attributes,
            content,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Html,
    Metadata,
    Title,
    Body,
    Div,
    Heading1,
    Heading2,
    Heading3,
    Paragraph,
    Link,
}

impl From<&str> for NodeKind {
    fn from(value: &str) -> Self {
        match value {
            "html" => NodeKind::Html,
            "meta" => NodeKind::Metadata,
            "title" => NodeKind::Title,
            "body" => NodeKind::Body,
            "div" => NodeKind::Div,
            "h1" => NodeKind::Heading1,
            "h2" => NodeKind::Heading2,
            "h3" => NodeKind::Heading3,
            "p" => NodeKind::Paragraph,
            "l" => NodeKind::Link,
            _ => panic!("Invalid node"),
        }
    }
}
