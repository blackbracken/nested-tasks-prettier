use std::collections::HashSet;

use crate::{
    parser::{assemble_tree, isolate_line},
    prettier::pretty_tree,
};

pub mod kernel;
pub mod parser;
pub mod prettier;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PrettyFlag {
    HideDetails { depth: u8 },
}

impl PrettyFlag {
    fn priority(&self) -> FlagPriority {
        match self {
            PrettyFlag::HideDetails { .. } => FlagPriority::Innocuity,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub enum FlagPriority {
    Innocuity,
}

pub fn pretty(input: Vec<String>, _: HashSet<PrettyFlag>) -> Vec<String> {
    let raw_nodes = input
        .iter()
        .map(|text| isolate_line(text.to_owned()))
        .collect::<Vec<_>>();

    let tree = assemble_tree(raw_nodes);

    pretty_tree(tree)
}
