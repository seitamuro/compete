use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }

    let ans = vec!["ABC", "ARC", "AGC", "AHC"];
    let ans: Vec<&str> = ans.into_iter().filter(|x| x.to_string().ne(&s1)).collect();
    let ans: Vec<&str> = ans.into_iter().filter(|x| x.to_string().ne(&s2)).collect();
    let ans: Vec<&str> = ans.into_iter().filter(|x| x.to_string().ne(&s3)).collect();

    println!("{}", ans[0]);
}
