use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : Chars
    }
    let mut ans = 0;
    for i in 0..n.len() {
        ans += n[i] as usize - '0' as usize;
        ans %= 9;
    }
    if ans == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
