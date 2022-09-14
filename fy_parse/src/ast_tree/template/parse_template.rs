use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use super::super::Block;
use regex::Regex;
use super::ast_template::HtmlAst;

static TAG_NAME: &str = r#"[a-zA-Z_][-.0-9_a-zA-Z]*"#;
static NAMESPACE_TAG_NAME: &str = r#"((?:[a-zA-Z_][-.0-9_a-zA-Z]*:)?[a-zA-Z_][-.0-9_a-zA-Z]*)"#;
static START_TAG: &str = r#"^<((?:[a-zA-Z_][-.0-9_a-zA-Z]*:)?[a-zA-Z_][-.0-9_a-zA-Z]*)"#;

static END_TAG: &str = r#"^</((?:[a-zA-Z_][-.0-9_a-zA-Z]*:)?[a-zA-Z_][-.0-9_a-zA-Z]*)[^>]*>"#;
static ATTRIBUTES: &str = r#"^\s*([^\s"'<>/=]+)(?:\s*(=)\s*(?:"([^"]*)"+|'([^']*)'+|([^\s"'=<>`]+)))?"#;
static START_TAG_CLOSE: &str = r#"^\s*(/?)>"#;
static DEFAULT_TAG_RE: &str = r#"\{\{((?:.|\r?\n)+?)\}\}"#;

impl Block<'_> {
    pub fn to_html_ast(&self) {
        // 整个模板数据
        let mut html_content = self.content.trim().to_string().clone();

        let mut tag_stack: Vec<Rc<RefCell<HtmlAst>>> = Vec::new();
        let mut ast_root: Option<Rc<RefCell<HtmlAst>>> = None;

        loop {
            if (&html_content).len() <= 0 {
                break;
            }

            let text_end = html_content.find("<");
            match text_end {
                Some(0) => {
                    match StartTag::parse_start_tag(&html_content) {
                        (new_content, Some(tag)) => {
                            html_content = new_content;
                            let mut start_ast = HtmlAst::normal_node(tag);
                            StartTag::put_into_ast(&mut ast_root, &mut tag_stack, start_ast);
                        },
                        (_, None) => {
                            // 如果是尖角号但是不是开始标签，则按结束标签匹配
                            html_content = EndTag::parse_end_tag(&html_content);
                            EndTag::remove_from_stack(&mut tag_stack);
                        },
                    }
                },
                Some(len) if len > 0 => {
                    let text = (&html_content[..len]).to_string().clone();
                    html_content = TextTag::eat(&html_content, len).to_string();
                    TextTag::handle_text(text, &mut tag_stack, &mut ast_root);
                },
                _ => {},
            }
        }

        println!("{:?}", ast_root);
    }
}

trait HtmlHandle {
    fn eat(content: &str, len: usize) -> &str {
        &content[len..]
    }
}

#[derive(Debug)]
pub struct StartTag {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

impl HtmlHandle for StartTag {}

impl StartTag {

    /// 匹配开始标签
    /// 包含开始标签和其属性
    fn parse_start_tag(html_content: &str) -> (String, Option<Self>) {
        let mut html_content = html_content.clone().to_string();
        let start_tag_re = Regex::new(START_TAG).unwrap();
        let start_tag_match = start_tag_re.captures(&html_content);

        if let Some(res) = start_tag_match {
            let tag_name = res.get(1).unwrap().as_str().to_string();
            let mut attributes: HashMap<String, String> = HashMap::new();

            // 删除匹配到的数据
            html_content = Self::eat(&html_content, res.get(0).unwrap().as_str().len()).to_string();

            // 解析开始标签的属性
            let start_tag_end_re = Regex::new(START_TAG_CLOSE).unwrap();
            let attribute_re = Regex::new(ATTRIBUTES).unwrap();
            loop {
                let start_tag_end_match = start_tag_end_re.captures(&html_content);
                if let Some(end) = start_tag_end_match {
                    html_content = Self::eat(&html_content, end.get(0).unwrap().as_str().len()).to_string();
                    break;
                }

                // 匹配属性
                if let Some(attr) = attribute_re.captures(&html_content) {
                    let key = attr.get(1).unwrap().as_str();
                    let value = match attr.get(3) {
                        Some(val) => val.as_str(),
                        None => match attr.get(4) {
                            Some(val) => val.as_str(),
                            None => match attr.get(5) {
                                Some(val) => val.as_str(),
                                None => "",
                            }
                        }
                    };
                    attributes.insert(key.to_string(), value.to_string());

                    // 删除匹配到的属性
                    html_content = Self::eat(&html_content, attr.get(0).unwrap().as_str().len()).to_string();
                } else {
                    break;
                }
            }

            (html_content, Some(Self {
                tag_name,
                attributes,
            }))
        } else {
            (html_content, None)
        }
    }

    fn put_into_ast(ast_root: &mut Option<Rc<RefCell<HtmlAst>>>, node_stack: &mut Vec<Rc<RefCell<HtmlAst>>>, current_ast: HtmlAst) {
        let mut current = Rc::new(RefCell::new(current_ast));
        if let None = ast_root {
            *(ast_root.borrow_mut()) = Some(Rc::clone(&current));
        }

        // 获取栈中最后一个元素 将当前的元素挂载到该元素下
        if node_stack.len() > 0 {
            let parent = &node_stack[node_stack.len() - 1];

            // 建立父子关系
            RefCell::borrow_mut(parent).children.push(Rc::clone(&current));
        }

        node_stack.push(Rc::clone(&current));
    }
}

#[derive(Debug)]
struct EndTag {}

impl HtmlHandle for EndTag {

}

impl EndTag {
    fn parse_end_tag(html_content: &str) -> String {
        let mut html_content = html_content.clone().to_string();

        let end_tag_re = Regex::new(END_TAG).unwrap();
        if let Some(end) = end_tag_re.captures(&html_content) {
            html_content = Self::eat(&html_content, end.get(0).unwrap().as_str().len()).to_string();
            html_content
        } else {
            html_content
        }
    }

    fn remove_from_stack(node_stack: &mut Vec<Rc<RefCell<HtmlAst>>>) {
        node_stack.pop();
    }
}


#[derive(Debug)]
struct TextTag {}

impl HtmlHandle for TextTag {}

impl TextTag {
    fn handle_text(text: String, node_stack: &mut Vec<Rc<RefCell<HtmlAst>>>, ast_root: &mut Option<Rc<RefCell<HtmlAst>>>) {
        let mut current_text = text;

        loop {
            if current_text.len() <= 0 {
                break;
            }

            // 匹配文本中的占位符
            let variable_re = Regex::new(DEFAULT_TAG_RE).unwrap();
            match variable_re.captures(&current_text) {
                Some(variable) => {
                    let start = variable.get(0).unwrap().start();
                    let len = variable.get(0).unwrap().end() - variable.get(0).unwrap().start();

                    // 如果在占位符之前有静态数据需要先将静态数据生成节点
                    if start > 0 {
                        Self::put_into_ast(&mut current_text[0..start].to_string(), node_stack, ast_root);
                        current_text = Self::eat(&current_text, start).to_string();
                    }

                    // 生成字面量节点 TODO 暂时用text_node替代
                    Self::put_into_ast(&mut current_text[0..len].to_string(), node_stack, ast_root);
                    current_text = Self::eat(&current_text, len).to_string();
                },
                None => {
                    // 未匹配到占位符
                    Self::put_into_ast(&mut current_text, node_stack, ast_root);
                    current_text = Self::eat(&current_text, current_text.len()).to_string();
                },
            };
        }
    }

    fn put_into_ast(current_text: &mut String, node_stack: &mut Vec<Rc<RefCell<HtmlAst>>>, ast_root: &mut Option<Rc<RefCell<HtmlAst>>>) {
        let text_node = HtmlAst::text_node(current_text.clone());
        let text_ref = Rc::new(RefCell::new(text_node));

        // 将节点放进ast树中
        let parent = &node_stack[node_stack.len() - 1];
        RefCell::borrow_mut(parent).children.push(text_ref);
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn regex() {
        let a = "dd<asdssds";
        let y = &a[2..];
        println!("{}", y);
    }
}
