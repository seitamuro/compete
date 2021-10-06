use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
        t: [String; n],
    }

    let mut ss :Vec<Vec<char>>= Vec::new();
    for i in s.iter() {
        ss.push(i.chars().collect());
    }

    let mut tt :Vec<Vec<char>>= Vec::new();
    for i in t.iter() {
        tt.push(i.chars().collect());
    }

    let s = cut(&ss);
    let mut t = tt;

    for _ in 0..4 {
        rotate(&mut t);

        let cut = cut(&t);
        let mut ans = true;
        for i in 0..s.len() {
            for j in 0..s.len() {
                if s[i][j] != cut[i][j] {
                    ans = false;
                }
            }
        }

        if ans {
            println!("Yes");
            return;
        }
    }

    println!("No");
/*
    let mut test = vec![
        ".....".chars().collect(),
        "..#..".chars().collect(),
        ".###.".chars().collect(),
        "..#..".chars().collect(),
        ".....".chars().collect()
    ];

    rotate(&mut test);
    dbg!(test.clone());

    let test = cut(&test);
    dbg!(test.clone());*/
}

fn rotate(s: &mut Vec<Vec<char>>) {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            result[j][s.len()-i-1] = s[i][j];
        }
    }

    *s = result;
}

fn cut(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; s.len()]; s.len()];
    let mut r = None;
    let mut c = None;
    for i in 0..s.len() {
        for j in 0..s.len() {
            if r == None && s[i][j] == '#' {
                r = Some(i);
            }

            if c == None && s[j][i] == '#' {
                c = Some(i);
            }
        }
        if r != None && c != None {
            break;
        }
    }

    let r = r.unwrap_or(0);
    let c = c.unwrap_or(0);

    for i in r..s.len() {
        for j in c..s.len() {
            result[i-r][j-c] = s[i][j];
        }
    }

    result
}