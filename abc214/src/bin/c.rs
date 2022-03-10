use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut ans = vec![std::usize::MAX; n];
    let mut _tt = Vec::new();
    for (i, &v) in t.iter().enumerate() {
        _tt.push((i, v));
    }
    _tt.sort_by(|x, y| x.1.cmp(&y.1));

    let mut tt = VecDeque::new();
    for i in _tt.into_iter() {
        tt.push_back(i);
    }

    //let mut ss = VecDeque::new();
    //for (i, v) in s.into_iter().enumerate() {
    //    ss.push_back((i, v));
    //}
    //ss.make_contiguous().sort_by(|x, y| x.1.cmp(&y.1));

    let (first, v) = tt.remove(0).unwrap();
    ans[first] = v;
    if first == n-1 {
        ans[0] = v + s[first];
    } else {
        ans[first+1] = v + s[first];
    }

    for i in first..n {
        if i == n-1 {
            ans[0] = (ans[i] + s[i]).min(t[0]);
        } else {
            ans[i+1] = (ans[i] + s[i]).min(t[i+1]);
        }
    }

    for i in 0..first {
        ans[i+1] = (ans[i] + s[i]).min(t[i+1]);
    }

    for i in ans.into_iter() {
        println!("{}", i);
    }
}
