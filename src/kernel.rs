use derive_new::new;

use self::task::{TaskTree, TreeNode};

pub mod task;
pub mod treatment {
    pub mod emojinize;
}

pub trait Treatment {
    fn treat(tree: TaskTree) -> TaskTree;
}

#[derive(new)]
struct EachNodeTreatment<F: FnOnce(TreeNode) -> TreeNode> {
    pub treat_each_node: F,
}

impl<F> Treatment for EachNodeTreatment<F>
where
    F: FnOnce(TreeNode) -> TreeNode,
{
    fn treat(tree: TaskTree) -> TaskTree {
        tree
    }
}
