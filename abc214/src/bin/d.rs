use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, usize); n-1],
    }

    let mut path = vec![vec![];n];
    for (u, v, w) in uvw.into_iter() {
        let u = u-1;
        let v = v-1;
        path[u].push((v, w));
        path[v].push((u, w));
    }

    let costs = solve(&path);
    let mut ans = 0;
    for i in 0..(n-1) {
        for j in (i+1)..n {
            ans += costs[j].max(costs[i]);
        }
    }

    println!("{}", ans);
}

fn solve(path: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut ans = vec![std::usize::MAX; path.len()];
    let mut todo = VecDeque::new();
    todo.push_back(0);
    ans[0] = 0;

    while !todo.is_empty() {
        let from = todo.remove(0).unwrap();
        for &(to, cost) in path[from].iter() {
            if ans[to] > cost {
                ans[to] = cost;
                todo.push_back(to);
            }
        }
    }

    ans
}