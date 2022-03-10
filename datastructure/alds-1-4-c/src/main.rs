use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut dict = HashMap::new();
    for _ in 0..n {
        let command = iter.next().unwrap();
        let key = iter.next().unwrap();
        if command == "insert" {
            dict.entry(key).or_insert(key);
        } else if command == "find" {
            if dict.get(key).is_none() {
                println!("no");
            } else {
                println!("yes")
            }
        }
    }
}