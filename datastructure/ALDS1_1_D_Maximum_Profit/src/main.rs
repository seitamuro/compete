use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut r: Vec<i64> = Vec::new();
    for i in iter {
        r.push(i.parse().unwrap());
    }

    let mut min = r[0];
    let mut ans = std::i64::MIN;

    for i in 1..n {
        ans = ans.max(r[i] - min);
        min = r[i].min(min);
    }

    println!("{}", ans);
}
