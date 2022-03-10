use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut query = vec![];
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                x: usize,
            }
            query.push((c, Some(x)));
        } else {
            query.push((c, None));
        }
    }

    let mut memo = VecDeque::new();
    
    /*
    push(1, &mut memo);
    push(1, &mut memo);
    push(0, &mut memo);
    push(4, &mut memo);
    push(3, &mut memo);
    push(6, &mut memo);
    push(5, &mut memo);

    let mut debug = vec![];
    for &i in memo.iter() {
        debug.push(i);
    }

    println!("{:?}", debug);
    
    */

    let mut new = VecDeque::new();
    for (c, x) in query.into_iter() {
        if c == 1 {
            new.push_back(x.unwrap());
        } else if c == 2 {
            if memo.len() > 0 {
                println!("{}", memo.remove(0).unwrap());
            } else {
                println!("{}", new.remove(0).unwrap());
            }
        } else {
            for i in new.into_iter() {
                push(i, &mut memo);
                if memo.len() == 2 {
                    let mut c = Vec::new();
                    for i in memo.into_iter() {
                        c.push(i);
                    }
                    c.sort();
                    memo = VecDeque::new();

                    for i in c.into_iter() {
                        memo.push_back(i);
                    }
                }
            }

            /*
            let mut debug = vec![];
            for &i in memo.iter() {
                debug.push(i);
            }
            */
            new = VecDeque::new();
        }
    }
}

fn push(x: usize, queue: &mut VecDeque<usize>) {
    let mut l = 0;
    let mut r = queue.len();

    //let mut d = 0;

    if queue.len() == 0 {
        queue.push_back(x);
        return;
    }

    while r - l > 1 {
        let d = (r - l) / 2 + l;

        if x <= queue[d] {
            r = d;
        } 
        
        if x > queue[d] {
            l = d;
        }
    }

    if queue.len() == 1 {
        if queue[0] < x {
            queue.insert(1, x);
        } else {
            queue.insert(0, x);
        }
    } else {
        if x < queue[l] {
            queue.insert(l, x);
        } else {
            queue.insert(r, x);
        }
    }
}