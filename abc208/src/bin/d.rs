#![allow(dead_code)]

use proconio::{input, fastout};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut path =vec![vec![]; n+1];
    for (a, b, c) in abc.into_iter() {
        path[a].push((b, c));
    }

    let mut memo = vec![];
    let node: Vec<usize> = (0..=n).collect();
    for i in 1..=n {
        memo.push(bfs2(i, &node, &path));
    }

    println!("{:?}", memo);
}

// path(to, cost)
fn bfs2(start: usize, node: &[usize], path: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
    let mut seen = vec![None; node.len()];
    let mut todo = Vec::new();

    seen[start] = Some(0);
    for (to, cost) in path[start].iter() {
        todo.push(*to);
        seen[*to] = Some(*cost);
    }

    while todo.len() != 0 {
        let from = todo.remove(0);

        for (to, cost) in path[from].iter() {
            let to = *to;
            let cost = *cost;

            if seen[to] == None {
                let x = seen[from].unwrap();
                seen[to] = Some(x + cost);
                todo.push(to);
            }
        }
    }

    seen
}


fn gcd(a: usize, b: usize) -> usize {
    let a = a % b;

    if a == 0 {
        return b;
    }

    let greater = a.max(b);
    let smaller = a.min(b);

    gcd(greater, smaller)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn search<T: Ord>(key: T, elems: &[T]) -> Option<usize> {
    let mut low = 0;
    let mut high = elems.len();

    while low != high {
        let m = (high - low) / 2 + low;

        match elems[m].cmp(&key) {
            Ordering::Equal => return Some(m),
            Ordering::Less => {
                if m == elems.len() - 1 {
                    return None;
                }

                low = m + 1;
            }
            Ordering::Greater => {
                if m == 0 {
                    return None;
                }

                high = m - 1;
            }
        }
    }

    if elems[low] == key {
        Some(low)
    } else {
        None
    }
}

// 幅優先探索
// startからの各ノードへの最短距離を返す
fn bfs(start: usize, node: &Vec<usize>, path: &Vec<Vec<usize>>) -> Vec<Option<usize>> {
    let mut ans = vec![None; node.len()];
    let mut todo = Vec::<usize>::new();

    ans[start] = Some(0);
    for &i in path[start].iter() {
        if ans[i] == None {
            let x = ans[start].unwrap();
            todo.push(i);
            ans[i] = Some(x + 1);
        }
    }

    while todo.len() != 0 {
        let i = todo.remove(0);

        for &node in path[i].iter() {
            if ans[node] == None {
                let x = ans[i].unwrap();
                todo.push(node);
                ans[node] = Some(x + 1);
            }
        }
    }

    ans
}

// 渡された数の約数を返す
fn get_divisors(n: usize) -> Vec<usize> {
    // https://algo-logic.info/divisor/
    let nsq = (n as f32).sqrt() as usize;
    let mut ans1 = Vec::new();
    let mut ans2 = Vec::new();

    for i in 1..nsq {
        if n % i == 0 {
            ans1.push(i);
            ans2.insert(0, n / i);
        }
    }

    if n % nsq == 0 {
        ans1.push(nsq);

        if n / nsq != nsq {
            ans1.push(n / nsq);
        }
    }

    ans1.append(&mut ans2);
    ans1
}
