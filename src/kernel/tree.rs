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

pub type Tree = Vec<Node>;

#[derive(Debug, Clone)]
pub struct NodeContent {
    pub label: String,
    pub status: Status,
}

#[derive(Debug, Clone, new)]
pub enum Node {
    Branch {
        depth: u8,
        task: NodeContent,
        children: Vec<Node>,
    },
    Leaf {
        depth: u8,
        task: NodeContent,
    },
}

impl Node {
    pub fn task(&self) -> &NodeContent {
        match self {
            Node::Branch {
                depth: _,
                task,
                children: _,
            } => task,
            Node::Leaf { depth: _, task } => task,
        }
    }

    pub fn add_children(self, mut nodes: Vec<Node>) -> Self {
        let node = self;

        match node {
            Node::Branch {
                depth,
                task,
                children,
            } => {
                let mut v = children;
                v.append(&mut nodes);

                Node::Branch {
                    depth,
                    task,
                    children: v,
                }
            }
            Node::Leaf { depth, task } => Node::Branch {
                depth,
                task,
                children: nodes.clone(),
            },
        }
    }

    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Branch {
                depth: _,
                task: _,
                children,
            } => Some(children),
            _ => None,
        }
    }

    pub fn depth(&self) -> u8 {
        match &self {
            Node::Branch {
                depth,
                task: _,
                children: _,
            } => *depth,
            Node::Leaf { depth, task: _ } => *depth,
        }
    }
}
