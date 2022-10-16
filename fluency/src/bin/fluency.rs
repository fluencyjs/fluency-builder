use fy_generator::script::script_traverse::ScriptGen;
use fy_generator::template::template_generator::TemplateGen;
use fy_parse::read_file::load;

fn main() {
  let (html_ast, script_ast, _) = load("web/test.fy", "web/bound.js");
  let file_name = "test";
  let mut modules: Vec<char> = file_name.chars().collect();
  modules[0] = modules[0].to_uppercase().nth(0).unwrap();
  let module_name: String = modules.into_iter().collect();

  // 数据更新埋点
  let mut script = ScriptGen::new(script_ast);
  script.traverse_ast();
  script.generate_code();
  // println!("{}", script.target_code);

  // 生成html逻辑代码
  if let Some(template_ast) = html_ast {
    let mut template = TemplateGen::new(template_ast, &module_name, script.response_variables.unwrap());
    template.generate_template_code();
  } else {
    return;
  }
}
