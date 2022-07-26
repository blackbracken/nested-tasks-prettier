use crate::kernel::{task::Tree, Treatment};

pub struct Emojinize {}

impl Treatment for Emojinize {
    fn treat(tree: Tree) -> Tree {
        tree
    }
}
