use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n]
    };

    let mut cnt1 = 0u64;
    let mut cnt2 = 0u64;
    let mut map = HashMap::new();

    for (i, j) in a.iter().zip(b.iter()) {
        if i == j {
            cnt1 += 1;
        }

        if map.contains_key(i) || map.contains_key(j) {
            cnt2 += 1;
        }

        map.entry(i).or_insert("");
        map.entry(j).or_insert("");
    }

    println!("{}\n{}", cnt1, cnt2);
}