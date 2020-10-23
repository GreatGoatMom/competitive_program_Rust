use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : usize,
        list : Chars
    }
    let mut ans = 0;
    let mut left_white = 0;
    let mut right_red = 0;
    for i in 0..a {
        if list[i] == 'R'{
            right_red += 1;
            ans += 1;
        }
    }
    for i in 0..a {
        if list[i] == 'W' {
            left_white += 1;
        } else {
            right_red -= 1;
        }
        ans = cmp::min(ans,cmp::max(left_white,right_red));
    }

    println!{"{}",ans};
}
