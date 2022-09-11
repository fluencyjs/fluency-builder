use project_root;
use fy_parse::read_file::FyBlock;

fn main() {
    let base_dir = match project_root::get_project_root() {
        Ok(path) => path,
        _ => panic!("the root path is empty!"),
    };

    // TODO 这里的文件需要做成配置
    let res_fy = FyBlock::read_file_to_block(&format!("{}/{}", base_dir.to_str().unwrap(), "test.fy"));
    let fy_obj = format!(r#"
        new Fluency({{
            webComponentName: '{}',
            template: `{}`,
            style: `{}`,
            script: `{}`,
        }});
    "#, res_fy.fy_id, res_fy.template_block, res_fy.style_block, res_fy.script_block);
    println!("{}", fy_obj);
}
