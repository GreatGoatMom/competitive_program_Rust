use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        temp: i32
    }
    let mut ans;
    ans = if temp >= 30 {
        "Yes"
    } else {
        "No"
    };
    println!{"{}", ans};
}
