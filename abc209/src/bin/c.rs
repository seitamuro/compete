#![allow(dead_code, non_snake_case, unused_imports)]
use proconio::input;
use std::cmp::Ordering;
use itertools::*;

const MOD: u64 = 1_000_000_000 + 7;

fn main() {
    input! {
        n: usize,
        mut c: [u64;n],
    }

    let mut ans = 1u64;
    c.sort_unstable();
    //println!("{:?}", c);

    c = c
        .into_iter()
        .enumerate()
        .map(|(y, x)| x.saturating_sub(y as u64))
        .collect();
    for i in c.iter() {
        ans *= i;
        ans %= MOD;
    }

    println!("{}", ans);
}

fn warshall_floyd(path: Vec<Vec<usize>>, cost: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut table = vec![vec![std::usize::MAX; path.len()]; path.len()];

    for i in 0..path.len() {
        table[i][i] = 0;
        for (j, c) in path[i].iter().zip(cost[i].iter()) {
            table[i][*j] = *c;
        }
    }

    for k in 0..path.len() {
        for i in 0..path.len() {
            for j in 0..path.len() {
                println!("{} -> {} | {} | {} {}", i, j, table[i][j], table[i][k], table[k][j]);
                table[i][j] = table[i][j].min(table[i][k].saturating_add(table[k][j]));
            }
        }
    }

    table
}

fn y() {
    println!("yes");
}

fn Y() {
    println!("Yes");
}

fn n() {
    println!("no");
}

fn N() {
    println!("No");
}

#[macro_export]
macro_rules! ans {
    ($i:item) => {
        println!("{}", stringify!($i))
    };
}

/// get_divisors function returns the list of divisors of n.
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

/// gcd function returns the greatest common factor of a and b.
/// For example, gcd(12, 7) returns 1.
fn gcd(a: usize, b: usize) -> usize {
    let a = a % b;

    if a == 0 {
        return b;
    }

    let greater = a.max(b);
    let smaller = a.min(b);

    gcd(greater, smaller)
}

/// lcm function returns the least common multiple of a and b.
/// For exmaple, gcd(4, 6) returns 24.
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

/// 幅優先探索
/// startからの各ノードへの最短距離を返す
/// path[i]はnode[i]が移動可能なノードを表す｡
fn bfs(start: usize, node: &[usize], path: &[Vec<usize>]) -> Vec<Option<usize>> {
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

    while !todo.is_empty() {
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

