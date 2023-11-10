use wasm_bindgen::prelude::*;
use pulldown_cmark::{html, Parser}; // parser 负责解析 Markdown 文本，将其转换为内部表示的 Markdown 元素，而 html 模块则负责将这些 Markdown 元素转换为 HTML 字符串

#[wasm_bindgen] // 标记导出给 wasm
pub fn parse_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new(); // 可变 string
    html::push_html(&mut html_output, parser); // 调用 html::push_html 函数
    html_output // 返回
}
