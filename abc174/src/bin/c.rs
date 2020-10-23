use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

fn main() {
    input!{
        x : usize
    }
    let mut div = vec![false;x+1];
    let mut num = 7;
    let mut ans = 1;
    for i in 0..x {
        num %= x;
        if num == 0 || div[num] {
            break;
        }
        div[num] = true;
        num = num * 10 + 7;
        ans += 1;
    }
    if div[num] {
        ans = -1;
    }
    println!{"{}",ans};
}
