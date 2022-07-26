use super::Treatment;
use crate::task::TaskTree;

pub struct Emojinize {}

impl Treatment for Emojinize {
    fn treat(tree: TaskTree) -> TaskTree {
        tree
    }
}
