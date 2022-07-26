use derive_new::new;

use crate::task::{Task, TaskTree, TreeNode};

pub mod emojinize;

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
