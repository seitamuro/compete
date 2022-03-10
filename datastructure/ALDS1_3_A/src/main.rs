use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.split_whitespace();
    let mut stack: Stack<String> = Stack::new();
    //let iter = "1 2 + 3 4 - *".split(" ");

    for i in iter {
        if "+-*/".contains(i) {
            let num2: i32 = stack.pop().unwrap().parse().unwrap();
            let num1: i32 = stack.pop().unwrap().parse().unwrap();
            stack.push(match i {
                "+" => format!("{}", num1 + num2),
                "-" => format!("{}", num1 - num2),
                "*" => format!("{}", num1 * num2),
                "/" => format!("{}", num1 / num2),
                _  => panic!("Unexpected operand!"),
            });
        } else {
            stack.push(i.to_string());
        }
    }

    println!("{}", stack.pop().unwrap());
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
        if self.length > 0 {
            self.length -=1;
            self.data.pop()
        } else {
            None
        }
    }
}