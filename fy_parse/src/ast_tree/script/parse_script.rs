use std::io::stderr;
use swc::Compiler;
use crate::ast_tree::Block;
use swc_common::{FileName, SourceMap};
use swc_common::errors::Handler;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, Syntax, TsConfig};


impl Block<'_> {
    pub fn to_script_ast(&self) -> ScriptAst {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".into()),
            self.content.trim().into(),
        );

        let compiler = Compiler::new(cm.clone());
        let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: false,
                decorators: true,
                dynamic_import: true,
                dts: false,
                no_early_errors: false,
                import_assertions: false,
            }),
            EsVersion::Es2022,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        ScriptAst::new(parser.parse_module().unwrap(), cm)
    }
}

pub struct ScriptAst {
    pub module: Module,
    pub cm: Lrc<SourceMap>,
}

impl ScriptAst {
    fn new(module: Module, cm: Lrc<SourceMap>) -> Self {
        Self {
            module,
            cm,
        }
    }
}
