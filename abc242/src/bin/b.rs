use proconio::input;

fn main() {
    input! {
        s: String
    };

    let mut numbers: Vec<u8> = s.chars().map(|x| x as u8).collect();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for i in numbers {
        print!("{}", i as char);
    }
}