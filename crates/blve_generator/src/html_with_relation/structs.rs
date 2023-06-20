use blve_html_parser::{Dom as RawDom, Element as RawElm, Node as RawNode};
use nanoid::nanoid;
use std::collections::HashMap;

pub struct Node {
    pub uuid: String,
    pub content: NodeContent,
}

pub enum NodeContent {
    Element(Element),
    TextNode(String),
    Comment(String),
}

pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, Option<String>>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new_raw(raw_elm: RawElm) -> Element {
        Element {
            attributes: raw_elm.attributes.clone(),
            children: vec![],
            tag_name: raw_elm.name,
        }
    }
}

impl Node {
    fn new_comment(comment: &String) -> Node {
        Node {
            uuid: nanoid!(),
            content: NodeContent::Comment(comment.clone()),
        }
    }

    fn new_text(text: &String) -> Node {
        Node {
            uuid: nanoid!(),
            content: NodeContent::TextNode(text.clone()),
        }
    }

    fn new_elm(elm: &RawElm) -> Node {
        let mut children = vec![];
        for child in &elm.children {
            children.push(Node::new_from_node(child));
        }
        Node {
            uuid: nanoid!(),
            content: NodeContent::Element(Element::new_raw(elm.clone())),
        }
    }

    pub fn new_from_dom(raw_dom: &RawDom) -> Result<Node, String> {
        match raw_dom.children.len() {
            0 => Err("Root element has no child".to_string()),
            1 => Ok(Node::new_from_node(&raw_dom.children[0])),
            _ => Err("Root element has more than one child".to_string()),
        }
    }

    pub fn new_from_node(raw_node: &RawNode) -> Node {
        match raw_node {
            RawNode::Text(text) => Node::new_text(text),
            RawNode::Element(elm) => Node::new_elm(elm),
            RawNode::Comment(comment) => Node::new_comment(comment),
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match &self.content {
            NodeContent::Element(elm) => elm.to_string(),
            NodeContent::TextNode(text) => text.clone(),
            NodeContent::Comment(comment) => comment.clone(),
        }
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        let mut attributes = String::new();
        for (key, value) in self.attributes.iter() {
            attributes.push_str(&format!(" {}=\"{}\"", key, value.as_ref().unwrap()));
        }
        let mut children = String::new();
        for child in &self.children {
            children.push_str(&child.to_string());
        }
        format!(
            "<{}{}>{}</{}>",
            self.tag_name, attributes, children, self.tag_name
        )
    }
}

mod tests {
    #[test]
    fn test_node_to_string() {
        let raw_html = "<div><p>hello</p></div>";
        let raw_node = blve_html_parser::Dom::parse(raw_html).unwrap();
        let el = raw_node.children[0].clone();
        let node = crate::html_with_relation::structs::Node::new_from_node(&el);
        assert_eq!(node.to_string(), raw_html);
    }
}
