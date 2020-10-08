use proconio::*;
use std::*;
use proconio::marker::{Bytes, Chars};

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : Chars
    }
    let s_len = s.len();
    if s[s_len-1] == 's' {
        println!("{}es",s.iter().collect::<String>());
    } else {
        println!("{}s",s.iter().collect::<String>());
    }
}
