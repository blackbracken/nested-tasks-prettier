use crate::task::{TaskTree, TreeNode};

impl TaskTree {
    pub fn pretty(&self) -> Vec<String> {
        self.nodes.iter().flat_map(|node| node.pretty()).collect()
    }
}

impl TreeNode {
    fn pretty(&self) -> Vec<String> {
        match self {
            TreeNode::Branch {
                depth: _,
                task: _,
                children,
            } => {
                let mut v = vec![self.pretty_single()];
                children
                    .iter()
                    .map(|child| child.pretty())
                    .flatten()
                    .for_each(|child| v.push(child));

                v
            }
            TreeNode::Leaf { depth: _, task: _ } => {
                vec![self.pretty_single()]
            }
        }
    }

    fn pretty_single(&self) -> String {
        let task = self.task();

        let tab = " ".repeat(self.depth().into());
        format!("{}- {} {}", tab, task.status.emoji(), task.content)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::assemble_tree;

    #[test]
    fn test_tree_pretty() {
        let raw_nodes = vec![
            (0, "- [x] 1".to_owned()),
            (0, "- [x] 2".to_owned()),
            (2, "- [x] 2-1".to_owned()),
            (2, "- [-] 2-2".to_owned()),
            (2, "- [>] 2-3".to_owned()),
            (4, "- [ ] 2-3-1".to_owned()),
            (0, "- [>] 3".to_owned()),
        ];
        let tree = assemble_tree(raw_nodes);

        let expected = r#"
- ✅ 1
- ✅ 2
  - ✅ 2-1
  - 🛑 2-2
  - 🚧 2-3
    - 📦 2-3-1
- 🚧 3
"#
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

        let actual = tree.pretty();

        assert_eq!(expected, actual);
    }
}
