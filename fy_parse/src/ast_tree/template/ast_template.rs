use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use super::parse_template::StartTag;

#[derive(Debug)]
pub struct HtmlAst {
    pub tag: Option<String>,
    pub attrs: Option<HashMap<String, String>>,
    pub text: Option<String>,
    pub children: Vec<Rc<RefCell<HtmlAst>>>,
    pub parent: Option<Rc<RefCell<HtmlAst>>>,
    pub node_type: AstType,
}

impl HtmlAst {
    pub fn normal_node(tag: StartTag) -> Self {
        Self {
            tag: Some(tag.tag_name),
            attrs: Some(tag.attributes),
            text: None,
            children: Vec::new(),
            parent: None,
            node_type: AstType::Normal,
        }
    }

    pub fn text_node(text: String) -> Self {
        Self {
            tag: None,
            attrs: None,
            text: Some(text),
            children: Vec::new(),
            parent: None,
            node_type: AstType::Text,
        }
    }
}

#[derive(Debug)]
pub enum AstType {
    Normal,
    Text,
}