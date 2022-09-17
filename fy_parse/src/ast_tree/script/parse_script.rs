use std::io::stderr;
use swc::Compiler;
use crate::ast_tree::Block;
use swc_common::{FileName, SourceMap};
use swc_common::errors::Handler;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, Syntax, TsConfig};


impl Block<'_> {
    pub fn to_script_ast(&self) {
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

        let list_error = parser.take_errors();
        if list_error.iter().len() > 0 {
            let mut err_msg = "".to_owned();
            for err in list_error {
                let msg = err.into_kind().msg().to_string();
                err_msg.push_str(msg.as_str());
            }
        }

        let mut module = parser.parse_module().unwrap();
        let s = serde_json::to_string_pretty(&module).expect("failed to serialize");
        println!("{}", s);
    }
}