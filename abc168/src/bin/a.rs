use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize
    }
    let ans = match n % 10 {
        3 => "bon",
        0 | 1 | 6 | 8 => "pon",
        _ => "hon"
    };
    println!("{}",ans);
}
