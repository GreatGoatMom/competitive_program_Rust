use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : i32,
        strList : [String; n]    
    }
    let mut cntList = vec![0;4];
    let aclist = ["AC","WA","TLE","RE"];
    for st in strList {
        for i in 0..4{
            if st==aclist[i] {
                cntList[i] += 1;
            }
        }
    }
    for i in 0..4 {
        println!{
            "{} x {}",
            aclist[i],
            cntList[i]
        };
    }
}
