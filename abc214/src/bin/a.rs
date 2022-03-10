use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n <= 125 {
        println!("4");
    } else if n <= 211 {
        println!("6");
    } else if n <= 214 {
        println!("8");
    }
}
