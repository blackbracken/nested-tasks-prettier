use std::collections::VecDeque;

use crate::task::{Status, TaskNode, TaskTree};

pub type RawNode = (u8, String);

enum HeadSpaceCount {
    UntilSpaces(u8),
    ReachedText(u8),
}

impl HeadSpaceCount {
    fn count(&self) -> u8 {
        match self {
            HeadSpaceCount::UntilSpaces(n) => *n,
            HeadSpaceCount::ReachedText(n) => *n,
        }
    }
}

pub fn isolate_line(raw_text: String) -> RawNode {
    let depth = raw_text
        .chars()
        .fold(
            HeadSpaceCount::UntilSpaces(0),
            |c: HeadSpaceCount, ch| match c {
                HeadSpaceCount::UntilSpaces(n) if ch.is_ascii_whitespace() => {
                    HeadSpaceCount::UntilSpaces(n + 1)
                }
                HeadSpaceCount::UntilSpaces(n) => HeadSpaceCount::ReachedText(n),
                _ => c,
            },
        )
        .count();

    let raw_text: String = raw_text.chars().skip(depth.into()).collect();

    return (depth, raw_text);
}

pub fn gen_tree(raw_nodes: Vec<RawNode>) -> TaskTree {
    let mut deque = VecDeque::from(raw_nodes);
    let nodes = parse_below_nodes(0, &mut deque);

    return TaskTree { nodes };
}

fn parse_below_nodes(current_depth: u8, deque: &mut VecDeque<(u8, String)>) -> Vec<TaskNode> {
    let mut nodes: Vec<TaskNode> = vec![];

    loop {
        match deque.front() {
            Some((depth, _)) if *depth == current_depth => {
                let (_, text) = deque.pop_front().unwrap();

                nodes.push(TaskNode::Leaf {
                    raw_text: text,
                    status: Status::Done,
                });
            }
            Some((depth, _)) if *depth == current_depth + 2 => {
                let children = parse_below_nodes(current_depth + 2, deque);
                let appended = nodes.pop().unwrap().add_children(children);

                nodes.push(appended);
            }
            _ => return nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("xxx", (0, String::from("xxx")))]
    #[test_case("  xxx", (2, String::from("xxx")))]
    fn test_isolate_line(text: &str, expected: RawNode) {
        let actual = isolate_line(text.to_owned());

        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_tree_empty() {
        
    }

    #[test]
    fn search_tree() {
        let mut v: VecDeque<(u8, String)> = VecDeque::from([
            (0, "-[ ] 0".to_owned()),
            (2, "-[ ] 2".to_owned()),
            (2, "-[ ] 2".to_owned()),
            (4, "-[ ] 4".to_owned()),
            (4, "-[ ] 4".to_owned()),
            (0, "-[ ] 0".to_owned()),
        ]);

        let x = parse_below_nodes(0, &mut v);

        println!("{:?}", x);
    }
}
