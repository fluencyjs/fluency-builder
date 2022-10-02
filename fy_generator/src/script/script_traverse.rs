use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::hash::Hash;
use swc::Compiler;
use swc::config::SourceMapsConfig;
use swc_common::{BytePos, Span, SyntaxContext, DUMMY_SP};
use swc_common::collections::AHashSet;
use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Stmt, Expr, BlockStmtOrExpr, PatOrExpr, ExprStmt, CallExpr, Ident, ExprOrSpread, Lit, Number, EsVersion, Callee};
use swc_common::util::take::Take;
use fy_parse::ast_tree::script::parse_script::ScriptAst;

pub struct ScriptGen {
  pub ast: ScriptAst,
  pub target_code: String,
}

impl ScriptGen {

  pub fn new(ast: ScriptAst) -> Self {
    Self {
      ast,
      target_code: String::from(""),
    }
  }

  pub fn traverse_ast(&mut self) {
    let s = serde_json::to_string_pretty(&self.ast.module).expect("failed to serialize");
    // println!("{}", s);

    // 找到所有声明的外部变量
    let mut responsive_variables: Vec<String> = Vec::new();
    for line in &self.ast.module.body {
      if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(variable))) = line {
        for declare in &variable.decls {
          if let Pat::Ident(ind) = &declare.name {
            let var_name = &ind.id.sym;
            responsive_variables.push(var_name.to_string());
          }
        }
      }
    }

    // 遍历ast上所有的节点
    for line in &mut self.ast.module.body {
      match line {
        ModuleItem::Stmt(node_content) => {
          Self::walk(node_content, None, &responsive_variables, |node, parent, responsive_var| {
            if let Stmt::Expr(expr) = &node {
              if let Expr::Assign(assign) = &expr.expr.borrow() {
                match &assign.left {
                  PatOrExpr::Pat(pat) => {
                    if let Pat::Ident(ident) = pat.borrow() {
                      let assign_variable = &ident.id.sym;
                      if responsive_var.contains(&assign_variable.to_string()) {
                        // 匹配到数据
                        // let response_expression = Self::gen_response_expr(expr.span.hi.0 + 1);
                        let Response(response_expression) = Response::gen_response_expr("$$response");
                        // node.take();
                        parent.unwrap().push(response_expression);
                      }
                    }
                  },
                  _ => {},
                }
              }
            }
          }, |node| {
            // println!("出去了{:?}", serde_json::to_string_pretty(&node).expect("failed to serialize"));
          });
        },
        _ => {},
      }
    }
  }

  pub fn generate_code(&mut self) {
    self.target_code = self.gen_new_script();
  }

  fn gen_new_script(&self) -> String {
    let compiler = Compiler::new(self.ast.cm.clone());
    let mut new_module = self.ast.module.clone() as Module;
    let new_source = compiler.print(
      &new_module,
      None,
      None,
      false,
      EsVersion::Es2022,
      SourceMapsConfig::Bool(false),
      &Default::default(),
      None,
      false,
      None,
      false,
      false,
    ).unwrap();

    new_source.code
  }

  pub fn walk(
    node: &mut Stmt,
    parent_node: Option<&mut Vec<Stmt>>,
    responsive_variables: &Vec<String>,
    enter: fn(current_node: &mut Stmt, parent: Option<&mut Vec<Stmt>>, responsive_variables: &Vec<String>) -> (),
    leave: fn(current_node: &mut Stmt) -> (),
  ) {
    enter(node, parent_node, responsive_variables);

    match node {
      Stmt::Expr(expr_stmt) => {
        match expr_stmt.expr.borrow_mut() {
          Expr::Call(call_expr) => {
            // 查看参数中是否有闭包 闭包中的数据需要遍历
            for arg in &mut call_expr.args {
              match arg.expr.borrow_mut() {
                Expr::Arrow(arrow_expr) => {
                  if let BlockStmtOrExpr::BlockStmt(body) = &mut arrow_expr.body {
                    for stmt in &mut body.stmts.clone() {
                      Self::walk(stmt, Some(&mut body.stmts), responsive_variables, enter, leave);
                    }
                  }
                },
                Expr::Fn(fn_expr) => {
                  if let Some(fn_body) = &mut fn_expr.function.body {
                    for stmt in &mut fn_body.stmts.clone() {
                      Self::walk(stmt, Some(&mut fn_body.stmts), responsive_variables, enter, leave);
                    }
                  }
                },
                _ => {},
              }
            }
          },
          _ => {},
        }
      },
      Stmt::Decl(decl_expr) => {
        match decl_expr {
          Decl::Fn(fn_expr) => {
            // 方法体中的数据需要继续遍历
            if let Some(fn_body) = &mut fn_expr.function.body {
              for stmt in &mut fn_body.stmts.clone() {
                Self::walk(stmt, Some(&mut fn_body.stmts), responsive_variables, enter, leave);
              }
            }
          },
          _ => {},
        }
      },
      _ => {},
    }

    leave(node);
  }

  fn print_module(node: &ModuleItem) {
    let s = serde_json::to_string_pretty(&node).expect("failed to serialize");
    println!("{}", s);
  }
}


struct Response(Stmt);

impl Response {
  pub fn gen_response_expr(response_func: &str) -> Self {
    let response_ind = Expr::Call(CallExpr {
      span: DUMMY_SP,
      callee: Callee::Expr(Box::new(Expr::Ident(Ident {
        span: DUMMY_SP,
        sym: response_func.into(),
        optional: false,
      }))),
      args: vec![
        ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: 0.0,
            raw: None,
          }))),
        },
        ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: 1.0,
            raw: None,
          }))),
        },
      ],
      type_args: None,
    });

    Response(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(response_ind),
    }))
  }
}