use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.split_whitespace();
    let mut numbers: Vec<usize> = Vec::new();
    for i in iter.skip(1) {
        numbers.push(i.parse().unwrap());
    }

    for i in 0..numbers.len() {
        for j in 0..i {
            if numbers[j] > numbers[i] {
                let mut tmp1 = numbers[i];
                let mut tmp2;
                for k in j..=i {
                    tmp2 = numbers[k];
                    numbers[k] = tmp1;
                    tmp1 = tmp2;
                }
                break;
            }
        }

        if i != 0 {
            println!("");
        }

        for (ind, j) in numbers.iter().enumerate() {
            if ind == 0 {
                print!("{}", j);
            } else {
                print!(" {}", j);
            }
        }
    }
}