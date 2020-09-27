use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        c : char
    }
    let ans = if c.is_uppercase() {
        "A"
    } else {
        "a"
    };
    println!{"{}", ans};
}
