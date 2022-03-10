use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    let a = s.chars().nth(n-1);

    if a == Some('o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
