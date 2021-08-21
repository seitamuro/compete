use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    if s.len() != 12 {
        println!("WA");
        return;
    }

    for (a, b) in s.chars().zip("Hello,World!".to_string().chars()) {
        if a != b {
            println!("WA");
            return;
        }
    }

    println!("AC");
}
