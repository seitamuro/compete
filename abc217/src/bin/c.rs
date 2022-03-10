use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut ans = vec![0; n];

    for (i, v) in p.into_iter().enumerate() {
        let i = i+1;
        let v = v-1;
        ans[v] = i;
    }

    for i in ans.into_iter() {
        println!("{} ", i);
    }
}
