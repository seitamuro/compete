use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let _n: usize= iter.next().unwrap().parse().unwrap();
    let q: usize= iter.next().unwrap().parse().unwrap();
    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
    let p: Vec<&str>= iter.collect();

    let mut cnt = 0;
    let mut elem1 = "";
    let mut elem2;
    for i in p.into_iter() {
        if cnt == 0 {
            elem1 = i;
            cnt += 1;
        } else {
            cnt = 0;
            elem2 = i;
            queue.push_back((elem1, elem2.parse().unwrap()));
        }
    }

    let mut elapsed = 0;
    while !queue.is_empty() {
        let (name, time) = queue.pop_front().unwrap();

        if time <= q {
            elapsed += time;
            println!("{} {}", name, elapsed);
        } else {
            queue.push_back((name, time - q));
            elapsed += q;
        }
    }
}
