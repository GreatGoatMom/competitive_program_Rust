use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        num_list : [i32;5]
    };
    for i in 0..5{
        if num_list[i] == 0 {
            println!("{}", i+1);
            break;
        }
    }
}
