use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; 2*n+m],
    }

    let mut hash = HashMap::new();
    let mut tree = vec![VecDeque::new(); m+1];
    //let mut memo = vec![false; n+1];
    let mut ans = 0u64;
    let mut k = 0;
    let mut ind = 0;
    for i in x.into_iter() {
        if k == 0 {
            k = i;
            ind += 1;
        } else {
            k -= 1;

            if k == 0 {
                solve(&mut hash, &mut tree, i, ind, &mut ans);
            } else {
                tree[ind].push_back(i);
            }
        }
    }

    let n = n as u64;
    if n*(1 + n)/2 == ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(hash: &mut HashMap<usize, usize>, tree: &mut [VecDeque<usize>], i: usize, ind: usize, ans: &mut u64) {
    let v = hash.get(&i);
    if v == None {
        hash.insert(i, ind);
    } else {
        let v = v.unwrap();
        *ans += i as u64;

        if tree[*v].len() > 0 {
            let next_i = tree[*v].pop_back().unwrap();
            let next_ind = *v;
            solve(hash, tree, next_i, next_ind, ans);
        }

        if tree[ind].len() > 0 {
            let next_i = tree[ind].pop_back().unwrap();
            let next_ind = ind;
            solve(hash, tree, next_i, next_ind, ans);
        }
    }
}