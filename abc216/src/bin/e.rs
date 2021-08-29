use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut b = 0;
    let mut ind = 1;
    let mut prev_v;
    prev_v = a[0];
    for (i,v) in a.iter().enumerate().skip(1) {
        b += v - prev_v;
        ind = i + 1;
        prev_v = *v;
        if b >= k {
            break;
        }
    }

    let mut ans = 0;
    if ind > 0 {
        let ind = ind-1;
        ans += a[ind-1] * b * ind - (b-1)*b/2*(k/ind)-k/ind*(k%ind);
    }
    let b = b-1;
    ans += a[ind-1] * (k-b) *ind- (b-1)*b/2*(k/ind)-k/ind*(k%ind);

    println!("{}", ans);
}
