#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;

fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        f: u64,
    }

    const MOD:u64 = 998244353u64;

    let a = a % MOD;
    let b = b % MOD;
    let c = c % MOD;
    let d = d % MOD;
    let e = e % MOD;
    let f = f % MOD;

    let mut m1;
    m1 = a*b;
    m1 %= MOD;
    m1 *= c;
    m1 %= MOD;

    let mut m2;
    m2 = d*e;
    m2 %= MOD;
    m2 *= f;
    m2 %= MOD;

    println!("{}", (m1 - m2 + MOD)%MOD);
}
