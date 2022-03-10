use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    for &i in a.iter() {
        if i != x {
            println!("{} ", i);
        }
    }
}