use super::{dom::NodeType, style::StyledNode};
use crate::style::{Display, PropertyMap};

#[derive(Debug, PartialEq)]
pub struct LayoutBox<'a> {
  pub box_type: BoxType<'a>,
  pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum BoxType<'a> {
  BlockBox(BoxProps<'a>),
  InlineBox(BoxProps<'a>),
  AnonymousBox,
}

#[derive(Debug, PartialEq)]
pub struct BoxProps<'a> {
  pub node_type: &'a NodeType,
  pub properties: PropertyMap,
}

pub fn to_layout_box<'a>(snode: StyledNode<'a>) -> LayoutBox<'a> {
  LayoutBox {
    box_type: match snode.display() {
      Display::Block => BoxType::BlockBox(BoxProps {
        node_type: snode.node_type,
        properties: snode.properties,
      }),
      Display::Inline => BoxType::InlineBox(BoxProps {
        node_type: snode.node_type,
        properties: snode.properties,
      }),
      Display::None => unreachable!(),
    },
    children: snode
      .children
      .into_iter()
      .fold(vec![], |mut acc: Vec<LayoutBox>, child| {
        match child.display() {
          Display::Block => {
            acc.push(to_layout_box(child));
            acc
          }
          Display::Inline => {
            match acc.last() {
              Some(&LayoutBox {
                box_type: BoxType::AnonymousBox,
                ..
              }) => {}
              _ => acc.push(LayoutBox {
                box_type: BoxType::AnonymousBox,
                children: vec![],
              }),
            };
            acc.last_mut().unwrap().children.push(to_layout_box(child));
            acc
          }
          Display::None => unreachable!(),
        }
      }),
  }
}