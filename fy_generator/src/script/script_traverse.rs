use std::borrow::Borrow;
use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Stmt, Expr, BlockStmtOrExpr};
use swc_common::util::take::Take;

pub struct ScriptGen {
    pub ast: Module,
}

impl ScriptGen {
    pub fn traverse_ast(&self) {
        let s = serde_json::to_string_pretty(&self.ast).expect("failed to serialize");
        // println!("{}", s);

        // 找到所有声明的外部变量
        let mut responsive_variables: Vec<String> = Vec::new();
        for line in &self.ast.body {
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
        for line in &self.ast.body {
            match line {
                ModuleItem::Stmt(node_content) => {
                    Self::walk(node_content, None, |node, parent| {}, |node, parent| {});
                },
                _ => {},
            }


            // Self::walk(line, None, |node, parent| {}, |node, parent| {});
        }

        // println!("{:?}", responsive_variables);
    }

    pub fn walk(
        node: &Stmt,
        parent_node: Option<&Stmt>,
        enter: fn(current_node: &Stmt, parent: Option<&Stmt>) -> (),
        leave: fn(current_node: &Stmt, parent: Option<&Stmt>) -> (),
    ) {
        enter(node, parent_node);

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
                                            Self::walk(&stmt, Some(node), enter, leave);
                                            println!("{:?}", serde_json::to_string_pretty(&stmt).expect("failed to serialize"));
                                        }
                                    }
                                },
                                Expr::Fn(fn_expr) => {
                                    if let Some(fn_body) = &fn_expr.function.body {
                                        for stmt in &fn_body.stmts {
                                            Self::walk(&stmt, Some(node), enter, leave);
                                            println!("{:?}", serde_json::to_string_pretty(&stmt).expect("failed to serialize"));
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
                                Self::walk(&stmt, Some(node), enter, leave);
                                println!("{:?}", serde_json::to_string_pretty(&stmt).expect("failed to serialize"));
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