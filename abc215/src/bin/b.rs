use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut r = n / 2;
    let mut ans = 0;
    while r != 0 {
        ans += 1;
        r /= 2;
    }

    println!("{}", ans);
}
