use derive_new::new;

use self::tree::{Tree, Node};

pub mod tree;
pub mod treatment {
    pub mod emojinize;
}

pub trait Treatment {
    fn treat(tree: Tree) -> Tree;
}

#[derive(new)]
struct EachNodeTreatment<F: FnOnce(Node) -> Node> {
    pub treat_each_node: F,
}

impl<F> Treatment for EachNodeTreatment<F>
where
    F: FnOnce(Node) -> Node,
{
    fn treat(tree: Tree) -> Tree {
        tree
    }
}
