use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n :usize,
        s : Chars
    }
    let lens = s.len();
    if lens > n {
        println!("{}...",s.iter().take(n).collect::<String>());
    } else {
        println!("{}",s.iter().collect::<String>());
    }
}
