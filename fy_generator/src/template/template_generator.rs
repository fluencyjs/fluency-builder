use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;

use fy_parse::ast_tree::template::ast_template::{AstType, HtmlAst};

pub struct TemplateGen<'a> {
  pub ast: Rc<RefCell<HtmlAst>>,
  pub module_name: &'a str,
}

impl<'a> TemplateGen<'a> {
  pub fn new(html_ast: Rc<RefCell<HtmlAst>>, module_name: &'a str) -> Self {
    Self {
      ast: html_ast,
      module_name,
    }
  }

  pub fn generate_template_code(&mut self) -> () {
    let mut tag_generator_map: HashMap<String, i32> = HashMap::new();
    let parent_node = RefCell::borrow(&self.ast);
    // let module_class = format!("class {}Module implements Module {{", self.module_name);
    // println!("{}", module_class);
    let class_field = String::from("");
    let create_elements = String::from("");

    let (field, create_element) = Self::convert_tag(&parent_node, &mut tag_generator_map);
    println!("field---{}", field);
    println!("element---{}", create_element);
    println!("{:?}", parent_node);
  }

  /// 转换tag标签为生成代码
  fn convert_tag(node: &HtmlAst, tag_generator_map: &mut HashMap<String, i32>) -> (String, String) {
    let mut field = String::from("");
    let mut element = String::from("");

    match node.node_type {
      AstType::Normal => {
        let tag_name = node.tag.as_ref().unwrap();
        let new_code = Self::get_generator_code(tag_name, tag_generator_map);

        field += &format!("{}{}?: ElementNode;", tag_name, new_code);
        element += &format!("this.{}{} = element('{}');", tag_name, new_code, tag_name);
      },
      AstType::Text => {
        let new_code = Self::get_generator_code("text", tag_generator_map);

        field += &format!("text{}?: ElementNode;", new_code);
        element += &format!("this.text{} = text('{}');", new_code, node.text.as_ref().unwrap());
      },
      AstType::Variable => {

      },
    };

    if node.children.len() > 0 {
      for node in &node.children {
        let current_node = RefCell::borrow(node);
        let (child_field, child_element) = Self::convert_tag(&current_node, tag_generator_map);
        field += &child_field;
        element += &child_element;
      }
    }

    (field, element)
  }

  /// 获取tag对应的序号
  fn get_generator_code(tag_name: &str, tag_generator_map: &mut HashMap<String, i32>) -> i32 {
    let mut new_code = 1;
    if let Some(generator_num) = tag_generator_map.get(tag_name) {
      new_code = generator_num + 1;
    }

    tag_generator_map.insert(tag_name.to_string(), new_code);
    new_code
  }
}