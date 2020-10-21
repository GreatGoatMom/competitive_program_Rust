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
        b : u128
    }
    let mut ans = 0;
    let mut tmp = 0;
    while x * a < y && x * a < x + b {
        tmp += 1;
        x *= a;
    }
    ans = tmp + ((y-x-1)/ b);
    println!{"{}",ans};
}
