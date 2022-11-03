#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;

fn main() {
    let mut buffers = Vec::new();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffers.push(buffer);
    for _ in 0..buffers[0].len()-2 {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        buffers.push(buffer);
    }

    let mut pos = Vec::new();
    for (x, buffer) in buffers.into_iter().enumerate() {
        for (y, c) in buffer.chars().enumerate() {
            if c == '#' {
                pos.push((x as isize, y as isize));
            }
        }
    }

    let mut cnt = 0;
    for p in (0..pos.len()).permutations(4) {
        let p1 = pos[p[0]];
        let p2 = pos[p[1]];
        let p3 = pos[p[2]];
        let p4 = pos[p[3]];

        let vx = p1.0 - p2.0;
        let vy = p1.1 - p2.1;

        // println!("{:?} {:?} {:?} {:?}", p1, p2, p3, p4);

        if vy != -(p1.0 - p3.0) {
            continue;
        }

        if vx != (p1.1 - p3.1) {
            continue;
        }

        if vx != (p3.0 - p4.0) {
            continue;
        }

        if vy != (p3.1 - p4.1) {
            continue;
        }

        //println!("{:?} {:?} {:?} {:?}", p1, p2, p3, p4);
        cnt += 1;
    }

    println!("{}", cnt / 4);
}
