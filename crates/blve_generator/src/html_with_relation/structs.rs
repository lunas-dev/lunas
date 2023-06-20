use blve_html_parser::{Dom as RawDom, Element as RawElm, Node as RawNode};
use nanoid::nanoid;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct Node {
    pub uuid: String,
    pub parent: Weak<RefCell<Node>>,
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
    pub children: RefCell<Vec<Rc<RefCell<Node>>>>,
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
    fn new_comment(comment: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            parent: Weak::new(),
            uuid: nanoid!(),
            content: NodeContent::Comment(comment),
        }))
    }

    fn new_text(text: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            parent: Weak::new(),
            uuid: nanoid!(),
            content: NodeContent::TextNode(text),
        }))
    }

    fn new_elm(elm: RawElm) -> Rc<RefCell<Node>> {
        let node_rc = Rc::new(RefCell::new(Node {
            parent: Weak::new(),
            uuid: nanoid!(),
            content: NodeContent::Element(Element::new_raw(elm.clone())),
        }));
        for child in &elm.children {
            Node::add_child(Rc::clone(&node_rc), Node::new_from_node(child.clone())).unwrap();
        }
        node_rc
    }

    pub fn new_from_dom(raw_dom: &RawDom) -> Result<Rc<RefCell<Node>>, String> {
        match raw_dom.children.len() {
            0 => Err("Root element has no child".to_string()),
            1 => Ok(Node::new_from_node(raw_dom.children[0].clone())),
            _ => Err("Root element has more than one child".to_string()),
        }
    }

    pub fn new_from_node(raw_node: RawNode) -> Rc<RefCell<Node>> {
        match raw_node {
            RawNode::Text(text) => Node::new_text(text),
            RawNode::Element(elm) => Node::new_elm(elm),
            RawNode::Comment(comment) => Node::new_comment(comment),
        }
    }

    fn add_child(parent: Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) -> Result<(), String> {
        match &parent.borrow().content {
            NodeContent::Element(elm) => {
                child.borrow_mut().parent = Rc::downgrade(&Rc::clone(&parent));
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
            children.push_str(&child.borrow().to_string());
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
        let node = crate::html_with_relation::structs::Node::new_from_node(el);
        assert_eq!(node.borrow().to_string(), raw_html);
    }
}
