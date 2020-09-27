use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n :i32,
        d :f64,
        numList : [[i64; 2];n]
    }
    let mut ans = 0;
    for itm in numList {
        let a = itm[0] * itm[0];
        let b = itm[1] * itm[1];
        let c = (a+b) as f64;
        if c.sqrt() <= d {
            ans += 1;
        }
    }
    println!{"{}", ans};
}
