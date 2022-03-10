#![allow(dead_code, non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        a: [isize;n]
    }

    let mut memo = x;
    //println!("{}", x);
    for (i, v) in a.iter().enumerate() {
        if i % 2 == 1 {
            memo -= v - 1;
        } else {
            memo -= v;
        }
        //println!("{}, {}", v, memo);
    }
    //println!("{}", memo);

    if memo >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
