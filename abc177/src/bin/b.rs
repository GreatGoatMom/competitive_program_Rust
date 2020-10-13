use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        s : Chars,
        t : Chars
    }
    let mut ans = 100_000_007;
    let s_len = s.len();
    let t_len = t.len();
    for i in 0..(s_len - t_len+1) {
        let mut tmp = 0;
        for j in 0..t_len{
            if s[i+j] != t[j] {
                tmp += 1;
            }
        }
        ans = cmp::min(ans,tmp);
    }
    println!("{}",ans);
}
