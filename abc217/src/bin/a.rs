use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    for (i, j) in s.chars().zip(t.chars()) {
        if i > j {
            println!("No");
            return;
        } else if i < j {
            println!("Yes");
            return;
        }
    }

    if s.len() > t.len() {
        println!("No");
    } else {
        println!("Yes");
    }
}
