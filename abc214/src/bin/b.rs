use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: usize,
        t: usize,
    }

    let mut ans = 0;
    for a in 0..=s {
        for b in 0..=(s-a) {
            for c in 0..=(s-a-b) {
                if a + b + c <= s && a*b*c <= t {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
