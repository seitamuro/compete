#![allow(dead_code)]

use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    }

    let mut ans = 0;
    for i in 0..(1 << k) {
        ans = ans.max(check(i, &ab, &cd));
    }

    println!("{}", ans);
}

fn check(mut mask: usize, ab: &[(usize, usize)], cd: &[(usize, usize)]) -> usize {
    let mut sara = vec![0; 101];
    for &(c, d) in cd.iter() {
        if mask % 2 == 0 {
            sara[c] += 1;
        } else {
            sara[d] += 1;
        }
        mask = mask / 2;
    }

    let mut ans = 0;
    for &(a, b) in ab.iter() {
        if sara[a] > 0 && sara[b] > 0 {
            ans += 1;
        }
    }

    ans
}

use std::cmp::Ordering;

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
