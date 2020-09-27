use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        x : usize,
        y : usize
    };
    let ans = if x * 2 <= y && x * 4 >= y && y % 2 == 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}",ans);
}
