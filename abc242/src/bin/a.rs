use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    };

    if x <= a {
        println!("1");
    } else if x <= b {
        println!("{}", 1. / (b - a) * c);
    } else {
        println!("0");
    }
}