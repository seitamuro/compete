#![allow(non_snake_case)]
use std::io::Read;
use std::collections::VecDeque;
use std::cmp::{Eq, PartialEq};
use std::iter::{IntoIterator, Iterator};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let _n: usize = iter.next().unwrap().parse().unwrap();
    let mut commands: Vec<(&str, usize)> = Vec::new();
    let inp: Vec<&str> = iter.collect();
    //let inp: Vec<&str> = "insert 5\ninsert 2\ninsert 3\ninsert 1\ndelete 3\ninsert 6\ndelete 5".split_whitespace().collect();

    let mut cnt = 0;
    let mut elem1 = "";
    let mut elem2;
    for i in inp.into_iter() {
        if cnt == 0 {
            elem1 = i; 

            if elem1 == "deleteFirst" {
                commands.push((elem1, 0));
            } else if elem1 == "deleteLast" {
                commands.push((elem1, 0));
            } else {
                cnt += 1;
            }
        } else {
            cnt = 0;
            elem2 = i;
            commands.push((elem1, elem2.parse().unwrap()));
        }
    }


    let mut list = List::new();
    for (cmd, i) in commands.into_iter() {
        match cmd {
            "insert" => list.insert(i),
            "delete" => list.delete(i),
            "deleteFirst" => list.deleteFirst(),
            "deleteLast" => list.deleteLast(),
            _ => panic!("Unexpected command"),
        }
    }

    for (i, v) in list.data.into_iter().enumerate() {
        if i == 0 {
            print!("{}", v);
        } else {
            print!(" {}", v);
        }
    }
    println!("");
}

struct List<T: Eq + PartialEq> {
    data: VecDeque<T>,
}

impl<T: Eq + PartialEq> List<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn insert(&mut self, x: T) {
        self.data.push_front(x);
    }

    fn delete(&mut self, x: T) {
        for (i, v) in self.data.iter().enumerate() {
            if *v == x {
                self.data.remove(i);
                return
            }
        }
    }

    fn deleteFirst(&mut self) {
        self.data.pop_front();
    }

    fn deleteLast(&mut self) {
        self.data.pop_back();
    }
}