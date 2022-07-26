use std::collections::HashSet;

use crate::{
    parser::{assemble_tree, isolate_line},
    prettier::pretty_tree,
};

pub mod parser;
pub mod prettier;
pub mod task;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Param {}

pub fn pretty(input: Vec<String>, _: HashSet<Param>) -> Vec<String> {
    let raw_nodes = input
        .iter()
        .map(|text| isolate_line(text.to_owned()))
        .collect::<Vec<_>>();

    let tree = assemble_tree(raw_nodes);

    pretty_tree(tree)
}
