use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input! {
        s : Chars,
        t : Chars
    }
    let slen = s.len();
    let tlen = t.len();
    let mut ans = 1_000;
    for s_index in 0..slen-tlen+1 {
        let mut tmp = 0;
        for add_index in 0..tlen {
            if s[s_index+add_index] != t[add_index]{
                tmp += 1;
            }
        }
        ans = cmp::min(ans,tmp);
    }
    println!{"{}", ans};
}
