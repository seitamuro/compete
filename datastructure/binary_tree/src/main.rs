use std::boxed::Box;

type ChildNode<T> = Option<Box<BTNode<T>>>;

macro_rules! AddNode {
    ($l:expr, $r:expr) => {
        BTNode::new(Op::Add, $l, $r);
    };
}

macro_rules! SubNode {
    ($l:expr, $r:expr) => {
        BTNode::new(Op::Sub, $l, $r);
    };
}

macro_rules! MulNode {
    ($l:expr, $r:expr) => {
        BTNode::new(Op::Mul, $l, $r);
    };
}

macro_rules! DivNode {
    ($l:expr, $r:expr) => {
        BTNode::new(Op::Div, $l, $r);
    };
}

macro_rules! IdNode {
    ($v:expr) => {
        BTNode::new(Op::Id($v), None, None);
    };
}

struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl BTNode<i32> {
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {
            op: op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

struct BinaryTree<T> {
    head: Option<BTNode<T>>
}

impl BinaryTree<i32> {
    pub fn new(head: BTNode<i32>) -> Self {
        BinaryTree::<i32> { head: Some(head) }
    }

    pub fn collapse(node: &Box<BTNode<i32>>) -> i32 {
        let mut r: Option<i32> = None;
        let mut l: Option<i32> = None;

        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }

        let r = if let Some(x) = r { x } else { 0 };
        let l = if let Some(x) = l { x } else { 0 };

        match node.op {
            Op::Add => { l + r }
            Op::Sub => { l - r }
            Op::Mul => { l * r }
            Op::Div => {
                if r == 0 {
                    panic!("attempted divide-by-zero operation.")
                }
                l / r
            }
            Op::Id(x) => x
        }
    }
}

enum Op<T> {
    Add,
    Sub,
    Div,
    Mul,
    Id(T)
}

fn main() {
    println!("Hello, world!");
}
