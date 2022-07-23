use std::collections::VecDeque;

use crate::task::{Status, Task, TaskTree, TreeNode};

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

pub fn assemble_tree(raw_nodes: Vec<RawNode>) -> TaskTree {
    let mut deque = VecDeque::from(raw_nodes);
    let nodes = parse_below_nodes(0, &mut deque);

    return TaskTree { nodes };
}

fn parse_below_nodes(current_depth: u8, deque: &mut VecDeque<(u8, String)>) -> Vec<TreeNode> {
    let mut nodes: Vec<TreeNode> = vec![];

    loop {
        match deque.front() {
            Some((depth, _)) if *depth == current_depth => {
                let depth = *depth;
                let (_, text) = deque.pop_front().unwrap();

                // 多重責務になるのでここで作成しない方が良い
                let task = make_task(text);

                nodes.push(TreeNode::Leaf { depth, task });
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

const PREFIX_LENGTH: usize = 5;
fn make_task(raw_text: String) -> Task {
    let prefix = raw_text.chars().take(PREFIX_LENGTH).collect::<String>();

    let status = Status::all()
        .iter()
        .find(|s| prefix == format!("- [{}]", s.ascii()))
        .unwrap()
        .clone();

    Task {
        content: raw_text.chars().skip(PREFIX_LENGTH + 1).collect(),
        status,
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

    #[test_case("- [x] done", 'x')]
    #[test_case("- [-] pending", '-')]
    #[test_case("- [>] doing", '>')]
    #[test_case("- [ ] new", ' ')]
    fn test_make_task_status_success(text: &str, expected: char) {
        let status = make_task(text.to_owned()).status;

        assert_eq!(status.ascii(), expected);
    }

    #[test]
    #[should_panic]
    fn test_make_task_status_failure() {
        make_task("- [?] unknown".to_owned());
    }

    #[test_case("- [x] aaa", "aaa")]
    #[test_case("- [x] 1 2 3", "1 2 3")]
    #[test_case(
        "- [x] 🚧あいうえおかきくけこさしすせそたちつてと",
        "🚧あいうえおかきくけこさしすせそたちつてと"
    )]
    fn test_make_task_content(text: &str, expected: &str) {
        let content = make_task(text.to_owned()).content;

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
            (0, "- [ ] 0".to_owned()),
            (2, "- [ ] 2".to_owned()),
            (2, "- [ ] 2".to_owned()),
            (4, "- [ ] 4".to_owned()),
            (4, "- [ ] 4".to_owned()),
            (6, "- [ ] 6".to_owned()),
            (2, "- [ ] 2".to_owned()),
            (0, "- [ ] 0".to_owned()),
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
        assert_eq!(not_nested.depth(), 2);
        assert!(matches!(not_nested.children(), None));

        // [0 -> [2, 2 -> [...], 2], 0]
        //           ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert_eq!(nested.depth(), 2);
        assert!(matches!(nested.children(), Some(children) if children.len() == 2));

        // [0 -> [2, 2 -> [4, 4 -> [...]], 2], 0]
        //                    ^
        let nested = nested.children().unwrap().get(1).unwrap();
        assert_eq!(nested.depth(), 4);
        assert!(matches!(nested.children(), Some(children) if children.len() == 1));

        // [0 -> [2, 2 -> [4, 4 -> [6]], 2], 0]
        //                          ^
        let nested = nested.children().unwrap().get(0).unwrap();
        assert_eq!(nested.depth(), 6);
        assert!(matches!(nested.children(), None));
    }
}
