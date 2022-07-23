#[derive(Debug, Clone, Copy)]
pub enum Status {
    Done,
    Pending,
    Doing,
    New,
}

impl Status {
    pub fn all() -> Vec<Self> {
        vec![Status::Done, Status::Pending, Status::Doing, Status::New]
    }

    pub fn ascii(&self) -> char {
        match self {
            Status::Done => 'x',
            Status::Pending => '-',
            Status::Doing => '>',
            Status::New => ' ',
        }
    }

    pub fn emoji(&self) -> char {
        match self {
            Status::Done => '✅',
            Status::Pending => '🛑',
            Status::Doing => '🚧',
            Status::New => '📦',
        }
    }
}

pub struct TaskTree {
    pub nodes: Vec<TreeNode>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub content: String,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub enum TreeNode {
    Branch { task: Task, children: Vec<TreeNode> },
    Leaf { task: Task },
}

impl TreeNode {
    pub fn task(&self) -> &Task {
        match self {
            TreeNode::Branch { task, children: _ } => task,
            TreeNode::Leaf { task } => task,
        }
    }

    pub fn add_children(self, mut nodes: Vec<TreeNode>) -> Self {
        let node = self;

        match node {
            TreeNode::Branch { task, children } => {
                let mut v = children;
                v.append(&mut nodes);

                TreeNode::Branch { task, children: v }
            }
            TreeNode::Leaf { task } => TreeNode::Branch {
                task,
                children: nodes.clone(),
            },
        }
    }

    pub fn children(&self) -> Option<&Vec<TreeNode>> {
        match self {
            TreeNode::Branch { task: _, children } => Some(children),
            _ => None,
        }
    }
}
