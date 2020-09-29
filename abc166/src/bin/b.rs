use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        k : usize,
    }
    let mut vec = vec![false; n];
    for _ in 0..k {
        input!{
            tmp : usize,
            lis : [usize; tmp]
        }
        for it in lis {
            vec[it - 1] = true;
        }
    }
    let mut ans = 0;
    for it in vec {
        if !it {
            ans += 1;
        }
    }
    println!("{}",ans);
}
