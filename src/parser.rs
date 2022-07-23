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
    use super::*;
    use test_case::test_case;

    #[test_case("zero", (0, String::from("zero")))]
    #[test_case("  tabbed", (2, String::from("tabbed")))]
    #[test_case("  tabbed spaces", (2, String::from("tabbed spaces")))]
    fn test_isolate_line(text: &str, expected: RawNode) {
        let actual = isolate_line(text.to_owned());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_gen_tree_empty() {
        let tree = gen_tree(vec![]);

        assert!(tree.nodes.is_empty());
    }

    #[test]
    fn test_gen_tree_nested() {
        let raw_nodes = vec![
            (0, "0".to_owned()),
            (2, "2".to_owned()),
            (2, "2".to_owned()),
            (4, "4".to_owned()),
            (4, "4".to_owned()),
            (6, "6".to_owned()),
            (2, "2".to_owned()),
            (0, "0".to_owned()),
        ];

        let tree = gen_tree(raw_nodes);

        let nodes = tree.nodes;
        assert_eq!(nodes.len(), 2);

        // [0 -> [...], 0]
        //  ^
        let nested = nodes.get(0).unwrap();
        assert!(matches!(nested.children(), Some(children) if children.len() == 3));

        // [0 -> [2, 2, 2], 0]
        //              ^
        let not_nested = nested.children().unwrap().get(2).unwrap();
        assert!(matches!(not_nested.children(), None));

        // [0 -> [2, 2 -> [...], 2], 0]
        //           ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert!(matches!(nested.children(), Some(children) if children.len() == 2));

        // [0 -> [2, 2 -> [4, 4 -> [...]], 2], 0]
        //                    ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert!(matches!(nested.children(), Some(children) if children.len() == 1));
    }
}
