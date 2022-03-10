use std::io::Read;
use std::collections::HashMap;

struct Tree {
    root: HashMap<usize, Node>
}

struct Node {
    id: usize,
    parent: Option<usize>,
    child: Vec<usize>
}

impl Tree {
    fn new() -> Self {
        let mut root = HashMap::new();
        root.entry(0).or_insert(Node {
            id: 0,
            parent: None,
            child: Vec::new()
        });
        Self { root }
    }

    fn add(&mut self, id: usize, child: Vec<usize>) {
        for i in child.iter() {
            let mut node = self.root.entry(*i).or_insert(Node {
                id: *i,
                parent: Some(id),
                child: Vec::new()
            });
            node.parent = Some(id);
        }
        let mut node = self.root.entry(id).or_insert(Node {
            id,
            parent: None,
            child: Vec::new()
        });
        node.id = id;
        node.child = child;
    }

    fn depth(&self, id: usize) -> usize {
        self._depth(id, 0)
    }

    fn _depth(&self, id: usize, depth: usize) -> usize {
        if self.root.get(&id).unwrap().parent.is_none() {
            depth
        } else {
            self._depth(self.root.get(&id).unwrap().parent.unwrap(), depth+1) 
        }
    }

    fn get(&self, id: usize) -> &Node {
        self.root.get(&id).unwrap()
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let mut tree = Tree::new();

    for _ in 0..n {
        let id = iter.next().unwrap().parse::<usize>().unwrap();
        let k = iter.next().unwrap().parse::<usize>().unwrap();
        let mut child = Vec::new();
        for _ in 0..k {
            child.push(iter.next().unwrap().parse::<usize>().unwrap())
        }

        tree.add(id, child);
    }

    for i in 0..n {
        let node = tree.get(i);
        let parent = if node.parent.is_none() { -1 } else { node.parent.unwrap() as i32};
        let depth = tree.depth(i);
        let child = node.child.clone();
        let state = if parent == -1 {
            "root"
        } else if child.len() > 0 {
            "internal node"
        } else {
            "leaf"
        };
        println!("node {}: parent = {}, depth = {}, {}, {:?}", i, parent, depth, state, child);
    }
}