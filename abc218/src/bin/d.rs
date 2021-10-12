use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        pos: [(u32, u32); n]
    }

    let mut x = HashMap::new();
    let mut y = HashMap::new();

    let mut cx = 0;
    let mut cy = 0;
    for (i, j) in pos.iter() {
        let i = *i as usize;

        //x[i as usize] += 1;
        *x.entry(i).or_insert(0) += 1;
        *y.entry(j).or_insert(0) += 1;

        if x[&i] == 2 {
            cx += 1;
        }
    }

    if cx == 0 {
        println!("{}", 0);
    } else {
        println!("{}", comb(cx, 2));
    }
}

fn comb(n: i32, r: i32) -> i32 {
     match (n, r) {
         (0, _) | (_, 0) => 1,
         _ => comb(n, r - 1) * (n - r + 1) / r	// ここで再帰呼び出し
     }
 }