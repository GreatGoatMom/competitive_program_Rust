use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n :usize,
        k :usize,
        mut fluits : [i32;n]
    }
    fluits.sort();
    let ans = fluits.iter().take(k).sum::<i32>();
    println!{"{}",ans};
}
