use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        mut x : u128,
        y : u128,
        a : u128,
        b : u128,
    }
    let mut ans : u128 = y / b;
    let mut cnt : u128 = 1;
    while a.overflowing_  ((a * x) as u128) < y {
        let tmpcnt : u128 = (y - (a * x)) / b;
        ans = cmp::max(ans, tmpcnt + cnt);
        cnt += 1;
        x *= a;
    }
    println!{"{}",ans};
}
