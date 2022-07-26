use crate::kernel::{task::TaskTree, Treatment};

pub struct Emojinize {}

impl Treatment for Emojinize {
    fn treat(tree: TaskTree) -> TaskTree {
        tree
    }
}
