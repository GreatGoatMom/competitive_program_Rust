use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : Chars,
        t : Chars
    }
    let mut ans = 0;
    let len = s.len();
    for i in 0..len {
        if s[i] != t[i] {
            ans += 1;
        }
    }
    println!{
        "{}",
        ans
    };
}
