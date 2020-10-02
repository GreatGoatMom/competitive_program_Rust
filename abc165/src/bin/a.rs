use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : usize,
        b : usize,
        c : usize
    }
    for i in b..c+1 {
        if i % a == 0 {
            println!{
                "OK"
            };
            return ;
        }
    }
    println!{"NG"};
}
