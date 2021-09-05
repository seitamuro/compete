use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }

    let mut ans = 0;
    for i in 0..=(2 * r as u64) {
        let i = i as f64 + x;
        if i == r {
            ans += ((r + (y - y.floor())).floor().abs() + (-r + y - y.floor() + 1.).floor().abs()) as usize + 1;
            continue;
        }

        let angle = (r / (r - i)).asin().abs();
        let up = (r * angle.sin() + y - y.floor()).floor();
        let down = (r * (-angle).sin() + y - y.floor()).floor() + 1.;
        ans += (up.abs() + down.abs()) as usize + 1;
    }

    println!("{}", ans);
}
