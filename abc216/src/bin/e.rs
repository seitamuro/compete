use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n],
    }

    a.push(0);
    a.sort();
    let mut prev_v = a[a.len()-1];
    let mut ans = 0;

    for (i, v) in a.iter().rev().enumerate().skip(1) {
        let i = i as u64;
        let diff = prev_v - v;

        let cnt = diff * i;
        if cnt <= k {
            ans += diff * (prev_v + v+1) / 2 * i;
            k -= cnt;
        } else {
            ans += k/i * (prev_v + prev_v - k/i+1) / 2 * i;
            ans += k%i * (prev_v - k/i);
            break;
        }

        if k == 0 {
            break;
        }

        prev_v = *v;
    }

    println!("{}", ans);
}
