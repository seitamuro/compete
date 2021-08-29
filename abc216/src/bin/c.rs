use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        mut n: u64,
    }

    let mut ans = VecDeque::new();
    while ans.len() < 120 {
        if n % 2 == 1 {
            n -= 1;
            ans.push_front("A");
        } else {
            n /= 2;
            ans.push_front("B");
        }
        
        if n == 0 {
            for i in ans.into_iter() {
                print!("{}", i);
            }
            return;
        }
    }

    panic!();
}
