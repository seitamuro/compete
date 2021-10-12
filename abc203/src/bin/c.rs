use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        mut ab: [(i64, i64); n],
    }

    let mut m = k;
    ab.sort_by(|a, b|a.cmp(b));
    let mut pos = 0;
    for (a, b) in ab.into_iter() {
        if a - pos <= m {
            m -= a - pos;
            pos += a - pos;
            m += b;
        } else {
            pos += m;
            m = 0;
            break;
        }
    }
    pos += m;
    println!("{}", pos);
}
