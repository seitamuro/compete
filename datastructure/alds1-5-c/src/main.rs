use std::io::Read;
use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    println!("0.0000000 0.00000000");
    koch(n, (0., 0.), (100., 0.));
    println!("100.0000000 0.00000000");
}

fn koch(n: usize, p1: (f64, f64), p2: (f64, f64)) {
    if n != 0 {
        let dx = (p2.0 - p1.0) / 3.;
        let dy = (p2.1 - p1.1) / 3.;

        let s = (p1.0 + dx, p1.1 + dy);
        let t = (p1.0 + dx * 2., p1.1 + dy * 2.);
        let top = (
            (t.0 - s.0) * (PI/3.).cos() - (t.1 - s.1) * (PI/3.).sin() + s.0,
            (t.0 - s.0) * (PI/3.).sin() + (t.1 - s.1) * (PI/3.).cos() + s.1
        );

        koch(n-1, p1, s);
        println!("{} {}", s.0, s.1);
        koch(n-1, s, top);
        println!("{} {}", top.0, top.1);
        koch(n-1, top, t);
        println!("{} {}", t.0, t.1);
        koch(n-1, t, p2);
    }
}
