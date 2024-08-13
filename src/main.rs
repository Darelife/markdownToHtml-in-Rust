use pulldown_cmark::{html, Parser};
use std::fs;
use std::path::Path;

extern crate pulldown_cmark;

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

const FILENAME: &str = "1.md";

fn main() {
    let filepath = format!("./markdown/{}", FILENAME);
    let path = Path::new(filepath.as_str());
    let markdown = fs::read_to_string(path).expect("Failed to read markdown file");

    let html = markdown_to_html(&markdown);

    let file_vector = FILENAME.split(".").collect::<Vec<&str>>();
    let mut output_path = "./html/".to_string();
    for i in 0..file_vector.len() - 1 {
        output_path.push_str(file_vector[i]);
    }
    output_path.push_str(".html");
    fs::write(output_path, html).expect("Failed to write HTML file");
}
