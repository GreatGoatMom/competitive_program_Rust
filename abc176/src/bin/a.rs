use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n :i32,
        x :i32,
        t :i32,
    }
    let mut divCnt = n / x;
    if n % x != 0 {
        divCnt += 1;
    }
    let ans = divCnt * t;
    println!("{}",ans);
}
