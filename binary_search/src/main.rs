use std::cmp::Ordering;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let a = [1, 5, 6, 10, 12, 20];
    let mut low = 0;
    let mut high = 5;
    let reader = BufReader::new(io::stdin());

    let key: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();

    while low != high {
        let m = (high - low) / 2 + low;
        match a[m].cmp(&key) {
            Ordering::Equal => {
                println!("{} is founded in a. The index is {}.", key, m);
                return;
            }
            Ordering::Greater => {
                if m == 0 {
                    break;
                }

                high = m - 1;
            }
            Ordering::Less => {
                if m == a.len() {
                    break;
                }

                low = m + 1;
            }
        }
    }

    println!("{} is not included in a.", key);
}
