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
        m : usize,
        x : usize,
        books : [[usize;m+1]; n]
    }
    let mut ans = 1_000_000_000;
    for bit in 0..=(1<<n) {
        let mut tmp_cnt = 0;
        let mut scores = vec![0;m];
        for i in 0..n {
            if (bit & 1<<i) != 0 {
                for point in 1..=m {
                    scores[point-1] += books[i][point];
                }
                tmp_cnt += books[i][0]
            }
        }
        let mut is_reach = true;
        for s in 0..m {
            if scores[s] < x {
                is_reach = false;
            }
        }
        if is_reach {
            ans = cmp::min(ans,tmp_cnt);
        }
    }
    if ans == 1_000_000_000 {
        println!("-1");
        return;
    }
    println!("{}",ans);
}
