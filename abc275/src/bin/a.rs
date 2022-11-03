#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;

fn main() {
    proconio::input! {
        n: usize,
        h: [usize; n],
    }

    let mut max = 0;
    let mut max_idx = None;
    for (i, &v) in h.iter().enumerate() {
        if max < v {
            max = v;
            max_idx = Some(i+1);
        }
    }

    println!("{}", max_idx.unwrap());
}
