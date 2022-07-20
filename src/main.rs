use std::{
    collections::VecDeque,
    io::{self, BufRead},
    thread::sleep,
    time::Duration,
};

enum HeadSpaceCount {
    UntilSpaces(u8),
    ReachedText(u8),
}

impl HeadSpaceCount {
    fn count(&self) -> u8 {
        match self {
            HeadSpaceCount::UntilSpaces(n) => *n,
            HeadSpaceCount::ReachedText(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Status {
    Done,
}

struct TaskTree {
    node: Vec<TaskNode>,
}

#[derive(Debug, Clone)]
enum TaskNode {
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
    fn add_children(self, nodes: &mut Vec<TaskNode>) -> Self {
        let node = self;
        match node {
            TaskNode::Node {
                raw_text,
                status,
                children,
            } => {
                let mut v = children;
                v.append(nodes);

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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let (spaces, content) = parse_input_line(line.unwrap());

        println!("{}: {}", spaces, content)
    }
}

fn parse_input_line(input: String) -> (u8, String) {
    let spaces = input.chars().fold(
        HeadSpaceCount::UntilSpaces(0),
        |c: HeadSpaceCount, ch| match c {
            HeadSpaceCount::UntilSpaces(n) if ch.is_ascii_whitespace() => {
                HeadSpaceCount::UntilSpaces(n + 1)
            }
            HeadSpaceCount::UntilSpaces(n) => HeadSpaceCount::ReachedText(n),
            _ => c,
        },
    );

    let content: String = input.chars().skip(spaces.count().into()).collect();

    (spaces.count(), content)
}

fn pairs_to_tree(deque: VecDeque<(u8, String)>) -> TaskTree {
    let mut deque = deque;

    let nodes = f(0, &mut deque);
    TaskTree { node: nodes }
}

fn f(current: u8, deque: &mut VecDeque<(u8, String)>) -> Vec<TaskNode> {
    let mut r: Vec<TaskNode> = vec![];

    loop {
        let peek = deque.front();
        println!("[{}] peeked: {:?}", current, peek);
        match peek {
            Some((n, _)) if *n == current => {
                println!("1st {}", n);
                let text = n.to_string();
                let (_, _) = deque.pop_front().unwrap();
                r.push(TaskNode::Leaf {
                    raw_text: text,
                    status: Status::Done,
                });
            }
            Some((n, _)) if *n == current + 2 => {
                println!("2nd {}", n);
                let last = r.pop().unwrap();
                let mut children = f(current + 2, deque);
                let last = last.add_children(&mut children);

                r.push(last);
            }
            _ => return r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let (spaces, content) = parse_input_line("".to_string());

        assert_eq!(spaces, 0);
        assert_eq!(content, "");
    }

    #[test]
    fn parse_tabbed() {
        let (spaces, content) = parse_input_line("  text".to_string());

        assert_eq!(spaces, 2);
        assert_eq!(content, "text");
    }

    #[test]
    fn search_tree() {
        let mut v: VecDeque<(u8, String)> = VecDeque::from([
            (0, "1".to_owned()),
            (2, "1".to_owned()),
            (2, "1".to_owned()),
            (4, "1".to_owned()),
            (4, "1".to_owned()),
            (0, "1".to_owned()),
        ]);

        let x = f(0, &mut v);

        println!("{:?}", x);
    }
}
