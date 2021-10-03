use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    //let mut buf = "5 3 8 1 7 3 9";
    let mut iter = buf.split_whitespace();

    let _ = iter.next().unwrap().parse::<usize>().unwrap();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut w = Vec::new();
    for i in iter {
        w.push(i.parse::<u64>().unwrap());
    }

    // binary search
    let mut left = 0;
    let mut right = 10000 * 100000;

    while right - left > 1 {
        let mid = (right - left) / 2 + left;
        if can_stack(mid, n, &w) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

fn can_stack(capacity: u64, num: usize, w: &[u64]) -> bool {
    let mut i = 0;

    for n in 0..num {
        let mut has = 0;
        while i < w.len() && has + w[i] <= capacity {
            has += w[i];
            i += 1;

        }

        if i == w.len() && n == num - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_stack1() {
        assert_eq!(false, can_stack(5, 1, &vec![10]));
        assert_eq!(false, can_stack(5, 1, &vec![6]));
        assert_eq!(true, can_stack(5, 1, &vec![5]));
        assert_eq!(true, can_stack(5, 1, &vec![3, 2]));
        assert_eq!(false, can_stack(5, 1, &vec![3, 3]));
        assert_eq!(false, can_stack(5, 1, &vec![5, 5]));
        assert_eq!(true, can_stack(5, 3, &vec![5, 5]));
        assert_eq!(true, can_stack(5, 2, &vec![5, 5]));
        assert_eq!(true, can_stack(5, 2, &vec![5]));
        assert_eq!(false, can_stack(1, 100, &vec![50]));
        assert_eq!(false, can_stack(4, 3, &vec![8, 1, 7, 3, 9]));
        assert_eq!(false, can_stack(5, 3, &vec![8, 1, 7, 3, 9]));
    }
}