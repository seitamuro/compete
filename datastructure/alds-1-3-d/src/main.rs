use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    //buf = r"\\///\_/\/\\\\/_/\\///__\\\_\\/_\/_/\".to_string();
    let mut stack = VecDeque::new();
    let mut ans = VecDeque::new();

    for (i, v) in buf.chars().enumerate() {
        if v == '\\' {
            stack.push_back(i);
        } else if v == '/' && !stack.is_empty() {
            let px = stack.pop_back().unwrap();
            let mut total_area = i - px;
            while !ans.is_empty() {
                let (pos, area) = ans.pop_back().unwrap();

                if pos >= px {
                    total_area += area;
                } else {
                    ans.push_back((pos, area));
                    break;
                }
            }
            ans.push_back((px, total_area));
        }
    }

    println!("{}", ans.iter().fold(0, |acc, (_, x)| acc + x));
    print!("{}", ans.len());
    for (_, area) in ans.iter() {
        print!(" {}", area);
    }
    println!("");
}
