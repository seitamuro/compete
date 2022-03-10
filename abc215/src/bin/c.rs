use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
        k: usize,
    }

    let mut s: Vec<char> = s.chars().collect();
    let mut h = HashMap::new();
    s.sort_by(|a, b| a.cmp(b));

    let len = s.len();
    let mut cnt = 0;
    for i in s.into_iter().permutations(len) {
        let i: String = i.into_iter().collect();
        match h.get(&i) {
            Some(_) => (),
            None => {
                cnt += 1;
                h.entry(i.clone()).or_insert(cnt);

                if cnt == k {
                    println!("{}", i);
                    return;
                }
            }
        }
    }
}
