use std::io::{self, BufRead};

use nested_tasks_prettier::parser::{gen_tree, isolate_line};

fn main() {
    let input = read_lines();
    let raw_nodes = input
        .iter()
        .map(|text| isolate_line(text.to_owned()))
        .collect::<Vec<_>>();
    let tree = gen_tree(raw_nodes);
}

fn read_lines() -> Vec<String> {
    let mut lines = io::stdin().lock().lines();
    let mut read = vec![];

    while let Some(Ok(text)) = lines.next() {
        read.push(text);
    }

    return read;
}
