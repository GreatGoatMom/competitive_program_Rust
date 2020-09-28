use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : i64,
        b : i64,
        c : i64,
        k : i64
    }
    let ans = if a >= k {
        k
    } else if a + b >= k {
        a
    } else {
        a - (k - a - b)
    };
    println!{"{}",ans};
}
