#[derive(Debug, Clone, Copy)]
pub enum Status {
    Done,
}

pub struct TaskTree {
    pub nodes: Vec<TaskNode>,
}

#[derive(Debug, Clone)]
pub enum TaskNode {
    Node {
        raw_text: String,
        status: Status,
        children: Vec<TaskNode>,
    },
    Leaf {
        raw_text: String,
        status: Status,
    },
}

impl TaskNode {
    pub fn add_children(self, mut nodes: Vec<TaskNode>) -> Self {
        let node = self;

        match node {
            TaskNode::Node {
                raw_text,
                status,
                children,
            } => {
                let mut v = children;
                v.append(&mut nodes);

                TaskNode::Node {
                    raw_text,
                    status,
                    children: v,
                }
            }
            TaskNode::Leaf { raw_text, status } => TaskNode::Node {
                raw_text,
                status,
                children: nodes.clone(),
            },
        }
    }
}
