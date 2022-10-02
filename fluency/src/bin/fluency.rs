use fy_generator::script::script_traverse::ScriptGen;
use fy_parse::read_file::load;

fn main() {
    let (html_ast, script_ast, _) = load("web/test.fy", "web/bound.js");
    let mut script = ScriptGen::new(script_ast);
    script.traverse_ast();
    script.generate_code();
    println!("{}", script.target_code);
}
