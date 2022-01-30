use std::{
    io::{self, BufWriter, Stdout, Write},
    num::ParseIntError,
};

fn input_number() -> Result<i32, ParseIntError> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<i32>();
}

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    let mut tree = BSTNode {
        value: input_number().unwrap(),
        left: None,
        right: None,
    };

    while let Ok(num) = input_number() {
        tree.insert(num);
    }

    tree.post_order(&mut sout);
}

pub struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

impl BSTNode {
    pub fn insert(&mut self, val: i32) {
        if val < self.value {
            match self.left {
                Some(ref mut node) => node.insert(val),
                None => self.left = Some(Box::new(BSTNode::leaf_node(val))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(val),
                None => self.right = Some(Box::new(BSTNode::leaf_node(val))),
            }
        }
    }

    pub fn post_order(self, sout: &mut BufWriter<Stdout>) {
        if let Some(node) = self.left {
            node.post_order(sout);
        }
        if let Some(node) = self.right {
            node.post_order(sout);
        }
        writeln!(sout, "{}", self.value).unwrap();
    }

    pub fn leaf_node(val: i32) -> BSTNode {
        BSTNode {
            value: val,
            left: None,
            right: None,
        }
    }
}
