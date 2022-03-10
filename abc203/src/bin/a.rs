use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if c == a {
        println!("{}", b);
    } else {
        println!("0");
    }
}
