use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut a = Vec::new();
    for _ in 0..n {
        let i = iter.next().unwrap().parse::<usize>().unwrap();
        a.push(i);
    }

    let q = iter.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..q {
        let m = iter.next().unwrap().parse::<usize>().unwrap();
        if solve(m, 0, 0, &a) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn solve(value: usize, sum: usize, ind: usize, num: &[usize]) -> bool {
    if sum == value || ind < num.len() && (solve(value, sum, ind+1, num) || solve(value, sum + num[ind], ind+1, num)) {
        return true;
    }

    false
}