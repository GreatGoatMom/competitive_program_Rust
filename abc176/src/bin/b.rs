use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input! {
        s : Chars
    }
    let mut ans = 0;
    for ind in 0..s.len() {
        ans += s[ind] as i32 - 48;
    }
    let ansStr = if ans % 9 == 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}",ansStr);
}
