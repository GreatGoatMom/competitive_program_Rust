use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        d : usize,
        list : [(isize,isize);n]
    }
    let mut ans = 0;
    for i in 0..n {
        let tmp = ((list[i].0*list[i].0 + list[i].1*list[i].1) as f64).sqrt();
        if tmp <= d as f64 {
            ans += 1;
        }
    }
    println!("{}",ans);
}
