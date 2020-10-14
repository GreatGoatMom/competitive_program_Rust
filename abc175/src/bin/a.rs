use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : Chars
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..3 {
        if s[i] == 'R' {
            tmp += 1;
        } else {
            ans = cmp::max(ans,tmp);
            tmp = 0;
        }
    }
    ans = cmp::max(ans,tmp);
    println!("{}",ans);
}
