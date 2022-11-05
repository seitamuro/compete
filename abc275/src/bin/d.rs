#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;

fn f(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 {
        return 1;
    }

    if memo.entry(n).or_insert(0) == &mut 0u64 {
        let v = f(n/2, memo) + f(n/3, memo);
        memo.insert(n, v);
    }

    *memo.get(&n).unwrap()
}
fn main() {
    proconio::input! {
        n: u64,
    }

    let mut memo: HashMap<u64, u64> = HashMap::new();

    println!("{}", f(n, &mut memo));
}
