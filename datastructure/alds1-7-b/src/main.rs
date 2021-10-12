use std::io::Read;
use std::boxed::Box;

struct Node {
    id: usize,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

struct BTree {
    root: Node,
}

impl BTree {
    pub fn new() -> Self {
        let root = Node {
            id: 0,
            right: None,
            left: None
        };

        Self {
            root
        }
    }

    pub fn add(&mut self, id: usize, right: Option<usize>, left: Option<usize>) {
        self.root.add(id, right, left);
    }

    pub fn get_parent(&self, id: usize) -> Option<usize> {
        self.root.get_parent(id)
    }

    pub fn get_sibling(&self, id: usize) -> Option<usize> {
        self.root.get_sibling(id)
    }

    pub fn get_degree(&self, id: usize) -> usize {
        let node = self.get(id).unwrap();
        node.get_degree()
    }

    pub fn get_depth(&self, id: usize) -> usize {
        self.root.get_depth(id)
    }

    pub fn get_type(&self, id: usize) -> &str {
        let node = self.get(id).unwrap();
        node.get_type()
    }

    pub fn get(&self, id: usize) -> Option<&Node> {
        self._get(&self.root, id)
    }

    pub fn get_height(&self, id: usize) -> usize {
        let node = self.get(id).unwrap();
        node.get_height()
    }

    pub fn _get<'a>(&self, node: &'a Node, id: usize) -> Option<&'a Node> {
        if node.id == id {
            return Some(node);
        } else {
            if node.right.is_some() {
                let ret = self._get(node.right.as_ref().unwrap(), id);

                if ret.is_some() {
                    return ret;
                }
            }

            if node.left.is_some() {
                let ret = self._get(node.left.as_ref().unwrap(), id);

                if ret.is_some() {
                    return ret;
                }
            }
        }

        None
    }
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            right: None,
            left: None
        }
    }

    pub fn get_type(&self) -> &str {
        if self.id == 0 {
            return "root";
        } else if self.get_degree() > 0 {
            return "internal node";
        } else {
            return "leaf";
        }
    }

    pub fn get_degree(&self) -> usize {
        let mut ans = 0;

        if self.right.is_some() {
            ans += 1;
        }

        if self.left.is_some() {
            ans += 1;
        }

        ans
    }

    pub fn get_sibling(&self, id: usize) -> Option<usize> {
        if self.right.is_some() {
            if self.right.as_ref().unwrap().id == id {
                return Some(self.left.as_ref().unwrap().id);
            }

            let ret = self.right.as_ref().unwrap().get_sibling(id);

            if ret.is_some() {
                return ret;
            }
        }

        if self.left.is_some() {
            if self.left.as_ref().unwrap().id == id {
                return Some(self.right.as_ref().unwrap().id);
            }

            let ret = self.left.as_ref().unwrap().get_sibling(id);

            if ret.is_some() {
                return ret;
            }
        }

        None
    }

    pub fn get_depth(&self, id: usize) -> usize {
        self._get_depth(id, 0)
    }

    fn _get_depth(&self, id: usize, depth: usize) -> usize {
        if self.id == id {
            return depth;
        } else {
            let mut d1 = None;
            let mut d2 = None;

            if self.left.is_some() {
                d1 = Some(self.left.as_ref().unwrap()._get_depth(id, depth + 1));
            }

            if self.right.is_some() {
                d2 = Some(self.right.as_ref().unwrap()._get_depth(id, depth + 1));
            }

            return d1.unwrap_or(std::usize::MAX).min(d2.unwrap_or(std::usize::MAX));
        }
    }

    pub fn get_height(&self) -> usize {
        self._get_height(0)
    }

    fn _get_height(&self, height: usize) -> usize {
        let mut h1 = None;
        let mut h2 = None;

        if self.right.is_some() {
            h1 = Some(self.right.as_ref().unwrap()._get_height(height + 1));
        }

        if self.left.is_some() {
            h2 = Some(self.left.as_ref().unwrap()._get_height(height + 1));
        }

        h1.unwrap_or(height).max(h2.unwrap_or(height))
    }

    pub fn get_parent(&self, id: usize) -> Option<usize> {
        if self.left.is_some() {
            if self.left.as_ref().unwrap().id == id {
                return Some(self.id);
            }

            let ret = self.left.as_ref().unwrap().get_parent(id);

            if ret.is_some() {
                return ret;
            }
        }

        if self.right.is_some() {
            if self.right.as_ref().unwrap().id == id {
                return Some(self.id);
            }

            let ret = self.right.as_ref().unwrap().get_parent(id);

            if ret.is_some() {
                return ret;
            }
        }

        None
    }

    pub fn add(&mut self, id: usize, right: Option<usize>, left: Option<usize>) {
        if self.id == id {
            if right.is_some() {
                self.right = Some(Box::new(Node::new(right.unwrap())));
            } else {
                self.right = None;
            }

            if left.is_some() {
                self.left = Some(Box::new(Node::new(left.unwrap())));
            } else {
                self.left = None;
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().add(id, right, left);
            }

            if self.left.is_some() {
                self.left.as_mut().unwrap().add(id, right, left);
            }
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
        let right = iter.next().unwrap().parse::<usize>().ok();
        let left = iter.next().unwrap().parse::<usize>().ok();
        tree.add(id, right, left);
    }

    for i in 0..n {
        let node = i;
        let parent = if let Some(x) = tree.get_parent(node) { x as i32 } else { -1 };
        let sibling = if let Some(x) = tree.get_sibling(node) {x as i32 } else { -1 };
        let degree = tree.get_degree(node);
        let depth = tree.get_depth(node);
        let height = tree.get_height(node);
        let node_type = tree.get_type(node);

        println!("node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}", node, parent, sibling, degree, depth, height, node_type);
    }
}