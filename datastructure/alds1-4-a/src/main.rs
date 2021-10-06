use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let n: usize;
    let mut s: Vec<usize> = Vec::new();
    let q: usize;
    let mut iter = buf.split_whitespace();

    n = iter.next().unwrap().parse().unwrap();
    for _ in 0..n {
        s.push(iter.next().unwrap().parse().unwrap());
    }
    //s.sort();

    let mut ans = 0;
    q = iter.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let key: usize = iter.next().unwrap().parse().unwrap();

        if b_search(&s, key).is_some() {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn b_search(src: &[usize], key: usize) -> Option<usize> {
    let mut left = 0;
    let mut right = src.len();
    let mut pmid = 0;
    let mut mid = 1;

    while pmid != mid {
        pmid = mid;
        mid = (right - left - (right - left)%2) / 2 + left;

        if src[mid] == key {
            return Some(mid);
        }

        if src[mid] > key {
            right = mid;
        } else if src[mid] <= key {
            left = mid;
        } 
    }

    None
}
