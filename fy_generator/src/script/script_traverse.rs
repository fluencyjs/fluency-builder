use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Stmt};

pub struct ScriptGen {
    pub ast: Module,
}

impl ScriptGen {
    pub fn traverse_ast(&self) {
        let s = serde_json::to_string_pretty(&self.ast).expect("failed to serialize");
        // println!("ast json is \n {}", s);

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
            Self::walk(line, None, |node, parent| {}, |node, parent| {});
        }

        println!("{:?}", responsive_variables);
    }

    pub fn walk(
        node: &ModuleItem,
        parent_node: Option<&ModuleItem>,
        enter: fn(current_node: &ModuleItem, parent: Option<&ModuleItem>) -> (),
        leave: fn(current_node: &ModuleItem, parent: Option<&ModuleItem>) -> (),
    ) {

    }
}