use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: [usize; 26],
    }

    let ascii: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in s.iter() {
        print!("{}", ascii[i-1]);
    }
}
