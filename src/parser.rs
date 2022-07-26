use std::collections::VecDeque;

use derive_new::new;

use crate::kernel::task::{Status, Task, TaskTree, TreeNode};

#[derive(PartialEq, Eq, Debug, new)]
pub struct RawNode {
    spaces: u8,
    text: String,
}

impl RawNode {
    fn depth(&self) -> u8 {
        self.spaces / 2
    }
}

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

    RawNode::new(depth, raw_text)
}

pub fn assemble_tree(raw_nodes: Vec<RawNode>) -> TaskTree {
    let mut deque = VecDeque::from(raw_nodes);
    // TODO: deque -> seq
    let nodes = parse_below_nodes(0, &mut deque);

    TaskTree { nodes }
}

// TODO: replace with functional combinators
fn parse_below_nodes(current_depth: u8, deque: &mut VecDeque<RawNode>) -> Vec<TreeNode> {
    let mut partial_tree: Vec<TreeNode> = vec![];

    loop {
        let root = match deque.front() {
            Some(peeked) if peeked.depth() == current_depth => {
                let popped = deque.pop_front().unwrap();

                let depth = popped.depth();
                let task = parse_task(popped.text);

                Some(TreeNode::new_leaf(depth, task))
            }
            Some(peeked) if peeked.depth() == current_depth + 1 => {
                let children = parse_below_nodes(current_depth + 1, deque);
                let parent = partial_tree.pop().unwrap().add_children(children);

                Some(parent)
            }
            _ => None,
        };

        if let Some(root) = root {
            partial_tree.push(root);
        } else {
            return partial_tree;
        };
    }
}

const PREFIX_LENGTH: usize = 5;
fn parse_task(raw_text: String) -> Task {
    let prefix = raw_text.chars().take(PREFIX_LENGTH).collect::<String>();

    let status = *Status::all()
        .iter()
        .find(|s| prefix == format!("- [{}]", s.ascii()))
        .unwrap();

    Task {
        content: raw_text.chars().skip(PREFIX_LENGTH + 1).collect(),
        status,
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("zero", RawNode::new(0, String::from("zero")))]
    #[test_case("  tabbed", RawNode::new(2, String::from("tabbed")))]
    #[test_case("  tabbed spaces", RawNode::new(2, String::from("tabbed spaces")))]
    fn test_isolate_line(text: &str, expected: RawNode) {
        let actual = isolate_line(text.to_owned());

        assert_eq!(expected, actual);
    }

    #[test_case("- [x] done", 'x')]
    #[test_case("- [-] pending", '-')]
    #[test_case("- [>] doing", '>')]
    #[test_case("- [ ] new", ' ')]
    fn test_make_task_status_success(text: &str, expected: char) {
        let status = parse_task(text.to_owned()).status;

        assert_eq!(status.ascii(), expected);
    }

    #[test]
    #[should_panic]
    fn test_make_task_status_failure() {
        parse_task("- [?] unknown".to_owned());
    }

    #[test_case("- [x] aaa", "aaa")]
    #[test_case("- [x] 1 2 3", "1 2 3")]
    #[test_case(
        "- [x] 🚧あいうえおかきくけこさしすせそたちつてと",
        "🚧あいうえおかきくけこさしすせそたちつてと"
    )]
    fn test_make_task_content(text: &str, expected: &str) {
        let content = parse_task(text.to_owned()).content;

        assert_eq!(content, expected)
    }

    #[test]
    fn test_assemble_tree_empty() {
        let tree = assemble_tree(vec![]);

        assert!(tree.nodes.is_empty());
    }

    #[test]
    fn test_assemble_tree_nested() {
        let raw_nodes = vec![
            RawNode::new(0, "- [ ] 0".to_owned()),
            RawNode::new(2, "- [ ] 2".to_owned()),
            RawNode::new(2, "- [ ] 2".to_owned()),
            RawNode::new(4, "- [ ] 4".to_owned()),
            RawNode::new(4, "- [ ] 4".to_owned()),
            RawNode::new(6, "- [ ] 6".to_owned()),
            RawNode::new(2, "- [ ] 2".to_owned()),
            RawNode::new(0, "- [ ] 0".to_owned()),
        ];

        let tree = assemble_tree(raw_nodes);

        let nodes = tree.nodes;
        assert_eq!(nodes.len(), 2);

        // [0 -> [...], 0]
        //  ^
        let nested = nodes.get(0).unwrap();
        assert_eq!(nested.depth(), 0);
        assert!(matches!(nested.children(), Some(children) if children.len() == 3));

        // [0 -> [2, 2, 2], 0]
        //              ^
        let not_nested = nested.children().unwrap().get(2).unwrap();
        assert_eq!(not_nested.depth(), 1);
        assert!(matches!(not_nested.children(), None));

        // [0 -> [2, 2 -> [...], 2], 0]
        //           ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert_eq!(nested.depth(), 1);
        assert!(matches!(nested.children(), Some(children) if children.len() == 2));

        // [0 -> [2, 2 -> [4, 4 -> [...]], 2], 0]
        //                    ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert_eq!(nested.depth(), 2);
        assert!(matches!(nested.children(), Some(children) if children.len() == 1));

        // [0 -> [2, 2 -> [4, 4 -> [6]], 2], 0]
        //                          ^
        let nested = nested.children().unwrap().get(0).unwrap();
        assert_eq!(nested.depth(), 3);
        assert!(matches!(nested.children(), None));
    }
}
