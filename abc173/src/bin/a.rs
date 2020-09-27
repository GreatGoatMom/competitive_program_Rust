use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : i32
    }
    let div = a % 1000;
    let ans = if div == 0 {
        0
    } else {
        1000 - div
    };
    println!{
        "{}",
        ans
    };
}
