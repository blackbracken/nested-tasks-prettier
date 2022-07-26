use derive_new::new;

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

pub type Tree = Vec<TreeNode>;

#[derive(Debug, Clone)]
pub struct Task {
    pub content: String,
    pub status: Status,
}

#[derive(Debug, Clone, new)]
pub enum TreeNode {
    Branch {
        depth: u8,
        task: Task,
        children: Vec<TreeNode>,
    },
    Leaf {
        depth: u8,
        task: Task,
    },
}

impl TreeNode {
    pub fn task(&self) -> &Task {
        match self {
            TreeNode::Branch {
                depth: _,
                task,
                children: _,
            } => task,
            TreeNode::Leaf { depth: _, task } => task,
        }
    }

    pub fn add_children(self, mut nodes: Vec<TreeNode>) -> Self {
        let node = self;

        match node {
            TreeNode::Branch {
                depth,
                task,
                children,
            } => {
                let mut v = children;
                v.append(&mut nodes);

                TreeNode::Branch {
                    depth,
                    task,
                    children: v,
                }
            }
            TreeNode::Leaf { depth, task } => TreeNode::Branch {
                depth,
                task,
                children: nodes.clone(),
            },
        }
    }

    pub fn children(&self) -> Option<&Vec<TreeNode>> {
        match self {
            TreeNode::Branch {
                depth: _,
                task: _,
                children,
            } => Some(children),
            _ => None,
        }
    }

    pub fn depth(&self) -> u8 {
        match &self {
            TreeNode::Branch {
                depth,
                task: _,
                children: _,
            } => *depth,
            TreeNode::Leaf { depth, task: _ } => *depth,
        }
    }
}
