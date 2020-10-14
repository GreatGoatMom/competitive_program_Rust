use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        t : isize
    }
    let mut ans ;
    if t >= 30 {
        ans = "Yes";
    } else {
        ans = "No";
    }
    println!("{}",ans);
}
