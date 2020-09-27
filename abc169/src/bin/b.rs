use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        x : usize,
        mut num_list : [i128;x]
    };
    num_list.sort();
    let mut ans = 1;
    for num in num_list.iter() {
        ans *= *num;
        if ans > 1_000_000_000_000_000_000 {
            println!("{}",-1);
            return ;
        }
    }
    println!("{}",ans);
}
