use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

fn main() {
    input!{
        k : usize
    }
    let mut list = vec![false;k+1];
    let mut ans = 0;
    let mut div = 0;
    for i in 0..k {
        ans += 1;
        div = div * 10 + 7 ;
        div %= k;
        if div == 0 {
            println!("{}",ans);
            return
        }
    }
    println!("-1");
}
