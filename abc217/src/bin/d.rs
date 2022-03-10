use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q],
    }

    let mut cut = VecDeque::new();
    for (c, x) in cx.into_iter() {
        if c == 2 {
            println!("{}", getlength(x, &cut, l));
        } else {
            push(x, &mut cut);
        }
    }
}

fn push(x: usize, queue: &mut VecDeque<usize>) {
    let mut l = 0;
    let mut r = queue.len();

    //let mut d = 0;
    while r - l > 1 {
        let d = (r - l) / 2 + l;

        if x <= queue[d] {
            r = d;
        } 
        
        if x >= queue[d] {
            l = d;
        }
    }

    queue.insert(r, x);
}

fn getlength(x: usize, cut: &VecDeque<usize>, l: usize) -> usize {
    let mut prev_i = 0;

    /*let mut debug = vec![];
    for &i in cut.iter() {
        debug.push(i);
    }*/

    for i in cut.iter() {
        let i = *i;
        if i > x {
            return i - prev_i;
        }
        prev_i = i;
    }

    l - prev_i
}
