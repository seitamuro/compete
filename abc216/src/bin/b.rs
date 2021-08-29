use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    for (i, (s1, t1)) in st.iter().enumerate() {
        for (j, (s2, t2)) in st.iter().enumerate() {
            if i == j { continue; }
            if s1.eq(s2) && t1.eq(t2) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
