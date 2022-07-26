// TODO: create struct

use crate::kernel::tree::{Node, Tree};

pub fn pretty_tree(tree: Tree) -> Vec<String> {
    tree.iter().flat_map(pretty_node).collect()
}

fn pretty_node(node: &Node) -> Vec<String> {
    match node {
        Node::Branch { children, .. } => {
            // TODO: consider to implement with only functional combinators
            let mut root = vec![pretty_node_once(node)];
            children
                .iter()
                .flat_map(pretty_node)
                .for_each(|child| root.push(child));

            root
        }
        Node::Leaf { .. } => {
            vec![pretty_node_once(node)]
        }
    }
}

fn pretty_node_once(node: &Node) -> String {
    let content = node.content();
    // TODO: newtype depth & spaces
    let tab = " ".repeat((2 * node.depth()).into());

    format!("{}- {} {}", tab, content.status.emoji(), content.label)
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{assemble_tree, RawNode},
        prettier::pretty_tree,
    };

    #[test]
    fn test_tree_pretty() {
        let raw_nodes = vec![
            RawNode::new(0, "- [x] 1".to_owned()),
            RawNode::new(0, "- [x] 2".to_owned()),
            RawNode::new(2, "- [x] 2-1".to_owned()),
            RawNode::new(2, "- [-] 2-2".to_owned()),
            RawNode::new(2, "- [>] 2-3".to_owned()),
            RawNode::new(4, "- [ ] 2-3-1".to_owned()),
            RawNode::new(0, "- [>] 3".to_owned()),
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

        let actual = pretty_tree(tree);

        assert_eq!(expected, actual);
    }
}
