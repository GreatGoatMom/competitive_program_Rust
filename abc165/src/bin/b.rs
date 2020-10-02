use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : i128
    }
    let mut ans = 0;
    let mut sum_num = 100;
    while sum_num < a {
        ans += 1;
        sum_num += sum_num / 100;
    }
    println!{
        "{}",
        ans
    };
}
