use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : String
    }
    let ans = if s == "ABC" {
        "ARC"
    } else {
        "ABC"
    };
    println!("{}",ans);
}
