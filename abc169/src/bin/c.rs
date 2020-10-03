use proconio::*;
#[warn(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
#[warn(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : usize,
        b : Chars 
    }
    let mut ans = 0;
    ans = a * (b[0] as usize - '0' as usize) * 100 + a * (b[2] as usize - '0' as usize) * 10 + a * (b[3] as usize - '0' as usize);
    println!("{}",ans/100);
}
