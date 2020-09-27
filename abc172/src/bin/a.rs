use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{a:i32}
    println!{"{}", a + a.pow(2) + a.pow(3)}
}
