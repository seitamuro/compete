use proconio::input;

fn solve(s: String) -> usize {
    let MOD: u64 = 998244353;
    let mut a: Vec<u64> = vec![];
    let mut b: Vec<u64> = vec![];

    let chars: Vec<usize> = s.chars().map(|x| x as usize).collect();

    let last = s.len() - 1;
    for i in 0..s.len() {
        let _s = i;
        let _e = last - _s;

        if _s > _e {
            break;
        }

        let _e1 = chars[_s] as u64;
        let _e2 = chars[_e] as u64;
        a.push(_s.max(_e));
    }

    let mut ans = 0;
    if a.len() > 0 {
        ans = 1;
        for i in a {
            ans *= i;
            ans %= MOD;
        }
    }

    for i in 0..(s.len()/2-1) {
        if chars[i] >= 1 {
            b.push(chars[i])
        }
    }

    if b.len() == 1 {
        ans += 26 * b[0];
    } else if b.len() > 1 {
        let mut _ans = 1;
        for i in b.iter() {
            
        }
    }

    0
}

fn main() {
    input! {
        t: usize,
        case: [(usize, String); t]
    };

    for (_n, s) in case.into_iter() {
        solve(s);
    }
}