use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : String,
        t : String
    }
    let slen = s.len();
    let ans = if s[..slen] == t[..slen] {
        "Yes"
    } else {
        "No"
    };
    println!("{}",ans);
}
