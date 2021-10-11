use std::io::Read;
use std::boxed::Box;
use std::cell::{RefCell, RefMut};

struct Node {
    id: usize,
    parent: Option<Box<RefCell<Node>>>,
    child: Vec<RefCell<Node>>,
}

struct BinaryTree {
    root: Node,
}

impl BinaryTree {
    fn new() -> Self {
        let root = Node {
            id: 0,
            parent: None,
            child: vec![]
        };

        Self {
            root
        }
    }

    fn is_exist(&self, id: usize) -> bool {
        self._is_exist(id, &self.root)
    }

    fn _is_exist(&self, id: usize, node: &Node) -> bool {
        if node.id == id {
            return true;
        } else {
            for i in node.child.iter() {
                if self._is_exist(id, &i.borrow()) {
                    return true;
                }
            }
        }

        false
    }

    fn get(&self, id: usize) -> Option<&RefCell<Node>> {
        self._get(id, &RefCell::new(self.root))
    }

    fn _get(&self, id: usize, node: &RefCell<Node>) -> Option<&RefCell<Node>> {
        if node.borrow().id == id {
            return Some(node);
        } else {
            for i in node.borrow().child.iter() {
                let ret = self._get(id, i);
                if ret.is_some() {
                    return ret;
                }
            }
        }

        None
    }
}

fn main() {
}