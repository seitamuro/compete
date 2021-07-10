use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a <= b && b <= a*6 {
        println!("Yes");
    } else {
        println!("No");
    }
}
