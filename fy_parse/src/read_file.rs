use std::fs;
use regex::Regex;
use std::path::Path;
use project_root;
use super::ast_tree::Block;

/// 解析入口
pub fn load(entry: &str, target: &str) {
    let base_dir = match project_root::get_project_root() {
        Ok(path) => path,
        _ => panic!("the root path is empty!"),
    };

    // 读取文件内容
    let res_fy = FyBlock::read_file_to_block(&format!("{}/{}", base_dir.to_str().unwrap(), entry));

    // 解析文件内容
    let template_block: Block = res_fy.template_block.as_str().into();
    template_block.to_html_ast();

    // let fy_obj = format!(r#"
    //     new Fluency({{
    //         webComponentName: 'f-{}',
    //         template: `{}`,
    //         style: `{}`,
    //     }});
    //     {}
    // "#, res_fy.fy_id, res_fy.template_block, res_fy.style_block, res_fy.script_block);
    // FyBlock::general_target(&format!("{}/{}", base_dir.to_str().unwrap(), target), fy_obj);
}

#[derive(Debug)]
pub struct FyBlock {
    pub fy_id: String,
    pub template_block: String,
    pub style_block: String,
    pub script_block: String,
}

impl FyBlock {

    /// 读取fy文件，将文件分为template style script块
    /// 如果block为空则返回空串
    pub fn read_file_to_block(file_path: &str) -> Self {
        let file_content = fs::read_to_string(file_path).unwrap();

        // 解析数据块闭包
        let parse = |regex: &str| -> String {
            let re = Regex::new(regex).unwrap();
            if let Some(cap) = re.captures(file_content.as_str()) {
                match cap.len() {
                    _len if _len >= 2 => cap.get(1).unwrap().as_str().to_string(),
                    _ => "".into(),
                }
            } else {
                "".into()
            }
        };

        let template_block = parse(r"<template>([\s\S]*)</template>");
        let style_block = parse(r"<style>([\s\S]*)</style>");
        let script_block = parse(r"<script>([\s\S]*)</script>");
        let fy_id = Path::new(file_path).file_stem().unwrap().to_str().unwrap().to_string();
        Self {
            fy_id,
            script_block,
            template_block,
            style_block,
        }
    }

    pub fn general_target(path: &str, fluency_object: String) {
        fs::write(path, fluency_object).unwrap();
    }
}
