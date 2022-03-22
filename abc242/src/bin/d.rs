use proconio::input;

fn next1(c: char) -> char {
    if c == 'A' {
        return 'B';
    } else if c == 'B' {
        return 'C';
    } else {
        return 'A';
    }
}

fn next2(c: char) -> char {
    if c == 'A' {
        return 'C';
    } else if c == 'B' {
        return 'A';
    } else {
        return 'B';
    }
}

fn solve(s: &String, t: u64, k: u64) -> char {
    if t == 0 {
        return s.chars().nth(k as usize).unwrap();
    } else {
        if k == 0 { 
            if t%3 == 0 {
                return s.chars().nth(0).unwrap();
            } else if t%3 == 1 { 
                return next1(s.chars().nth(0).unwrap());
            } else {
                return next2(s.chars().nth(0).unwrap());
            }
        } 

        if k%2 == 0 {
            return next1(solve(s, t-1, k/2));
        } else {
            return next2(solve(s, t-1, k/2));
        }
    }
}

fn main() {
    input! {
        s: String,
        q: usize,
        tk: [(u64, u64); q]
    };

    for (t, k) in tk.into_iter() {
        println!("{}", solve(&s, t, k-1));
    }
}