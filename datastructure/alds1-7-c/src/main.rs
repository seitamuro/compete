use std::collections::HashMap;
use std::io::Read;

struct BTree {
    tree: HashMap<usize, Node>,
}

#[derive(Debug)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: usize, left: Option<usize>, right: Option<usize>) {
        let mut entry = self.tree.entry(id).or_insert(Node::new());
        entry.left = left;
        entry.right = right;

        if left.is_some() {
            let mut entry = self.tree.entry(left.unwrap()).or_insert(Node::new());
            entry.parent = Some(id);
        }

        if right.is_some() {
            let mut entry = self.tree.entry(right.unwrap()).or_insert(Node::new());
            entry.parent = Some(id);
        }
    }

    pub fn preorder(&self, id: usize) {
        let node  = self.tree.get(&id).unwrap();

        print!(" {}", id);

        if node.left.is_some() {
            self.preorder(node.left.unwrap());
        }

        if node.right.is_some() {
            self.preorder(node.right.unwrap());
        }
    }

    pub fn inorder(&self, id: usize) {
        let node = self.tree.get(&id).unwrap();

        if node.left.is_some() {
            self.inorder(node.left.unwrap());
        }

        print!(" {}", id);

        if node.right.is_some() {
            self.inorder(node.right.unwrap());
        }
    }

    pub fn postorder(&self, id: usize) {
        let node = self.tree.get(&id).unwrap();

        if node.left.is_some() {
            self.postorder(node.left.unwrap());
        }

        if node.right.is_some() {
            self.postorder(node.right.unwrap());
        }

        print!(" {}", id);
    }

    pub fn get_root(&self) -> usize {
        let start = self.tree.keys().next().unwrap();

        self._get_root(*start)
    }

    fn _get_root(&self, id: usize) -> usize {
        let entry = self.tree.get(&id).unwrap();

        if entry.parent.is_none() {
            id
        } else {
            self._get_root(entry.parent.unwrap())
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            parent: None,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut tree = BTree::new();
    for _ in 0..n {
        let id = iter.next().unwrap().parse::<usize>().unwrap();
        let left = iter.next().unwrap().parse::<usize>().ok();
        let right = iter.next().unwrap().parse::<usize>().ok();

        tree.add(id, left, right);
    }

    let root = tree.get_root();

    // preoder
    println!("Preorder");
    tree.preorder(root);
    println!("");

    // inorder
    println!("Inorder");
    tree.inorder(root);
    println!("");

    // postorder
    println!("Postorder");
    tree.postorder(root);
    println!("");
}