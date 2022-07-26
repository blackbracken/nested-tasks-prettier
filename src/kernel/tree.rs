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
        content: NodeContent,
        children: Vec<Node>,
    },
    Leaf {
        depth: u8,
        content: NodeContent,
    },
}

impl Node {
    pub fn content(&self) -> &NodeContent {
        match self {
            Node::Branch {
                depth: _,
                content,
                children: _,
            } => content,
            Node::Leaf { depth: _, content } => content,
        }
    }

    pub fn add_children(self, mut nodes: Vec<Node>) -> Self {
        let node = self;

        match node {
            Node::Branch {
                depth,
                content,
                children,
            } => {
                let mut v = children;
                v.append(&mut nodes);

                Node::Branch {
                    depth,
                    content,
                    children: v,
                }
            }
            Node::Leaf { depth, content } => Node::Branch {
                depth,
                content,
                children: nodes.clone(),
            },
        }
    }

    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Branch {
                depth: _,
                content: _,
                children,
            } => Some(children),
            _ => None,
        }
    }

    pub fn depth(&self) -> u8 {
        match &self {
            Node::Branch {
                depth,
                content: _,
                children: _,
            } => *depth,
            Node::Leaf { depth, content: _ } => *depth,
        }
    }
}
