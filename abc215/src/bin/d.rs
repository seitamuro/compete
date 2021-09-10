use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    
    let mut nums = VecDeque::new();
    for i in 2..=m {
        nums.push_back(i);
    }

    let mut primes = VecDeque::new();
    while !nums.is_empty() {
        let d = nums.remove(0).unwrap();
        primes.push_back(d);
        nums = nums.into_iter().filter(|x| x % d != 0).collect();
    }

    let mut rl = Vec::new();
    while !primes.is_empty() {
        let p = primes.remove(0).unwrap();
        for i in a.iter() {
            if i % p == 0 {
                rl.push(p);
                break;
            }
        }
    }

    let mut ans = VecDeque::new();
    for i in 1..=m {
        ans.push_back(i);
    }

    for i in rl.iter() {
        ans = ans.into_iter().filter(|x| x % i != 0).collect();
    }

    println!("{}", ans.len());
    for i in ans.iter() {
        println!("{}", i);
    }
}
