use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=k {
            ans += 100 * i + j;
        }
    }
    println!("{}", ans);
}
