use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        map: [String; h],
    }

    let map: Vec<Vec<char>> = map.into_iter().map(|x| x.chars().collect()).collect();

    let mut ans = 0;
    for i in 0..(h-1) {
        for j in 0..(w-1) {
            let mut cnt = 0;
            cnt += if map[i][j] == '#' {1} else {0};
            cnt += if map[i+1][j] == '#' {1} else {0};
            cnt += if map[i][j+1] == '#' {1} else {0};
            cnt += if map[i+1][j+1] == '#' {1} else {0};

            if cnt % 2 == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
