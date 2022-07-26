use std::io::{self, BufRead};

use nested_tasks_prettier::{
    parser::{assemble_tree, isolate_line},
    prettier::pretty_tree,
};

fn main() {
    let input = read_lines();
    let raw_nodes = input
        .iter()
        .map(|text| isolate_line(text.to_owned()))
        .collect::<Vec<_>>();

    let tree = assemble_tree(raw_nodes);

    let output = pretty_tree(tree).join("\n");
    print!("{}", output);
}

fn read_lines() -> Vec<String> {
    let mut lines = io::stdin().lock().lines();
    let mut read = vec![];

    while let Some(Ok(line)) = lines.next() {
        read.push(line);
    }

    read.into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect()
}
