use build_browser::{
    css,
    dom::{Node, NodeType},
    html,
    layout::to_layout_box,
    render::to_element_container,
    style::to_styled_node,
};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let css_filename = &args[2];

    let mut f = File::open(filename).expect("file not found");
    let mut html_contents = String::new();
    f.read_to_string(&mut html_contents)
        .expect("something went wrong reading the file");

    let mut f_css = File::open(css_filename).expect("file not found");
    let mut css_contents = String::new();
    f_css.read_to_string(&mut css_contents)
        .expect("something went wrong reading the file");
    
    let mut siv = cursive::default();

    let node = html::parse(&html_contents);
    let stylesheet = css::parse(format!(
        "{}\n{}",
        &css_contents,
        collect_tag_inners(&node, "style".into()).join("\n")
    ));

    let container = to_styled_node(&node, &stylesheet)
        .and_then(|styled_node| Some(to_layout_box(styled_node)))
        .and_then(|layout_box| Some(to_element_container(layout_box)));
    if let Some(c) = container {
        siv.add_fullscreen_layer(c);
    }

    siv.run();
}

pub fn collect_tag_inners(node: &Box<Node>, tag_name: &str) -> Vec<String> {
    if let NodeType::Element(ref element) = node.node_type {
        if element.tag_name.as_str() == tag_name {
            return vec![node.inner_text()];
        }
    }

    node.children
        .iter()
        .map(|child| collect_tag_inners(child, tag_name))
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect()
}
