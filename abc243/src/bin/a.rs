use proconio::input;

fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize
    };

    let mut v = Some(v);
    let mut cnt = 0i32;

    loop {
        let consume = match cnt%3 {
            0 => a,
            1 => c,
            2 => b,
            _ => panic!(),
        };

        v = v.unwrap().checked_sub(consume);

        if v.is_none() {
            let user = match cnt%3 {
                0 => "F",
                1 => "M",
                2 => "T",
                _ => panic!(),
            };
            println!("{}", user);
            break;
        }

        cnt += 1;
    }
}