use std::fs;
use regex::Regex;
use std::path::Path;

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
}
