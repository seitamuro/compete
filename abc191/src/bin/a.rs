use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }

    if v*t <= d && d <= s*v {
        println!("No");
    } else {
        println!("Yes");
    }
}
