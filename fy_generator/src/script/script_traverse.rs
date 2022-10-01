use std::borrow::Borrow;
use swc::Compiler;
use swc::config::SourceMapsConfig;
use swc_common::{BytePos, Span, SyntaxContext};
use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Stmt, Expr, BlockStmtOrExpr, PatOrExpr, ExprStmt, CallExpr, ExprOrSuper, Ident, ExprOrSpread, Lit, Number, EsVersion};
use swc_common::util::take::Take;
use fy_parse::ast_tree::script::parse_script::ScriptAst;

pub struct ScriptGen {
  pub ast: Box<ScriptAst>,
}

impl ScriptGen {
  pub fn traverse_ast(&self) {
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
    for line in &self.ast.module.body {
      match line {
        ModuleItem::Stmt(node_content) => {
          Self::walk(node_content, None, &responsive_variables, |node, parent, responsive_var| {
            if let Stmt::Expr(expr) = node {
              if let Expr::Assign(assign) = &expr.expr.borrow() {
                match &assign.left {
                  PatOrExpr::Pat(pat) => {
                    if let Pat::Ident(ident) = pat.borrow() {
                      let assign_variable = &ident.id.sym;
                      if responsive_var.contains(&assign_variable.to_string()) {
                        println!("匹配到了！！！");
                        let response_expression = Self::add_response(expr.span.hi.0 + 1);

                      }
                      // println!("出去了{:?}", serde_json::to_string_pretty(&ident).expect("failed to serialize"));
                    }
                  },
                  _ => {},
                }
              }
            }
          }, |node, parent| {
            // println!("出去了{:?}", serde_json::to_string_pretty(&node).expect("failed to serialize"));
          });
        },
        _ => {},
      }


      // Self::walk(line, None, |node, parent| {}, |node, parent| {});
    }

    // println!("{:?}", responsive_variables);


    println!("{}", self.gen_new_script());
  }

  fn add_response(start: u32) -> ModuleItem {
    let response_func = String::from("$$response");

    let response_ind = Expr::Call(CallExpr {
      span: Span::new(BytePos(start), BytePos(start + response_func.len() as u32 - 2u32), SyntaxContext::empty()),
      callee: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident {
        span: Span::new(BytePos(start), BytePos(start + response_func.len() as u32), SyntaxContext::empty()),
        sym: response_func.clone().into(),
        optional: false,
      }))),
      args: vec![
        ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Lit(Lit::Num(Number {
            span: Span::new(BytePos(start + response_func.len() as u32 + 1u32), BytePos(start + response_func.len() as u32 + 2u32), SyntaxContext::empty()),
            value: 0.0,
          }))),
        },
        ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Lit(Lit::Num(Number {
            span: Span::new(BytePos(start + response_func.len() as u32 + 4u32), BytePos(start + response_func.len() as u32 + 5u32), SyntaxContext::empty()),
            value: 1.0,
          }))),
        },
      ],
      type_args: None,
    });

    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: Span::new(BytePos(start), BytePos(start + response_func.len() as u32 + 3u32), SyntaxContext::empty()),
      expr: Box::new(response_ind),
    }))
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
    ).unwrap();

    new_source.code
  }

  pub fn walk(
    node: &Stmt,
    parent_node: Option<&Stmt>,
    responsive_variables: &Vec<String>,
    enter: fn(current_node: &Stmt, parent: Option<&Stmt>, responsive_variables: &Vec<String>) -> (),
    leave: fn(current_node: &Stmt, parent: Option<&Stmt>) -> (),
  ) {
    enter(node, parent_node, responsive_variables);

    match node {
      Stmt::Expr(expr_stmt) => {
        match expr_stmt.expr.borrow() {
          Expr::Call(call_expr) => {
            // 查看参数中是否有闭包 闭包中的数据需要遍历
            for arg in &call_expr.args {
              match arg.expr.borrow() {
                Expr::Arrow(arrow_expr) => {
                  if let BlockStmtOrExpr::BlockStmt(body) = &arrow_expr.body {
                    for stmt in &body.stmts {
                      Self::walk(&stmt, Some(node), responsive_variables, enter, leave);
                    }
                  }
                },
                Expr::Fn(fn_expr) => {
                  if let Some(fn_body) = &fn_expr.function.body {
                    for stmt in &fn_body.stmts {
                      Self::walk(&stmt, Some(node), responsive_variables, enter, leave);
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
            if let Some(fn_body) = &fn_expr.function.body {
              for stmt in &fn_body.stmts {
                Self::walk(&stmt, Some(node), responsive_variables, enter, leave);
              }
            }
          },
          _ => {},
        }
      },
      _ => {},
    }

    leave(node, parent_node);
  }

  fn print_module(node: &ModuleItem) {
    let s = serde_json::to_string_pretty(&node).expect("failed to serialize");
    println!("{}", s);
  }
}