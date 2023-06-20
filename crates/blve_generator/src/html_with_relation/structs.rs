use blve_html_parser::{Element as RawElm, Node as RawNode};
use nanoid::nanoid;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

struct Node {
    uuid: String,
    parent: RefCell<Option<Weak<Node>>>,
    content: NodeContent,
}

enum NodeContent {
    Element(Element),
    TextNode(String),
    Comment(String),
}

struct Element {
    tag_name: String,
    attributes: HashMap<String, Option<String>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Element {
    pub fn new_raw(raw_elm: RawElm) -> Element {
        Element {
            attributes: raw_elm.attributes.clone(),
            children: RefCell::new(vec![]),
            tag_name: raw_elm.name,
        }
    }
}

impl Node {
    fn new_comment(comment: String) -> Rc<Node> {
        Rc::new(Node {
            parent: RefCell::new(None),
            uuid: nanoid!(),
            content: NodeContent::Comment(comment),
        })
    }

    fn new_text(text: String) -> Rc<Node> {
        Rc::new(Node {
            parent: RefCell::new(None),
            uuid: nanoid!(),
            content: NodeContent::TextNode(text),
        })
    }

    fn new_elm(elm: RawElm) -> Rc<Node> {
        let node_rc = Rc::new(Node {
            parent: RefCell::new(None),
            uuid: nanoid!(),
            content: NodeContent::Element(Element::new_raw(elm.clone())),
        });
        for child in &elm.children {
            Node::add_child(Rc::clone(&node_rc), Node::new_from(child.clone())).unwrap();
        }
        node_rc
    }

    fn new_from(raw_node: RawNode) -> Rc<Node> {
        match raw_node {
            RawNode::Text(text) => Node::new_text(text),
            RawNode::Element(elm) => Node::new_elm(elm),
            RawNode::Comment(comment) => Node::new_comment(comment),
        }
    }

    fn add_child(parent: Rc<Node>, child: Rc<Node>) -> Result<(), String> {
        match &parent.as_ref().content {
            NodeContent::Element(elm) => {
                child
                    .parent
                    .borrow_mut()
                    .replace(Rc::downgrade(&Rc::clone(&parent)));
                elm.children.borrow_mut().push(child);
                Ok(())
            }
            _ => return Err("Parent is not an element".to_string()),
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
        for child in self.children.borrow().iter() {
            children.push_str(&child.to_string());
        }
        format!(
            "<{}{}>{}</{}>",
            self.tag_name, attributes, children, self.tag_name
        )
    }
}

mod tests {
    use super::*;
    use blve_html_parser::Dom;

    #[test]
    fn test_node_to_string() {
        let raw_html = "<div><p>hello</p></div>";
        let raw_node = Dom::parse(raw_html).unwrap();
        let el = raw_node.children[0].clone();
        let node = Node::new_from(el);
        assert_eq!(node.to_string(), raw_html);
    }
}
