use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.split_whitespace();

    for i in iter {
    }
}

struct Stack<T> {
    length: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, elm: T) {
        self.data.push(elm);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.length -= 1;
    }
}