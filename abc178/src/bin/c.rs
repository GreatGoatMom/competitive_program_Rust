use proconio::*;
use std::*;

const MOD: isize = 1_000_000_007;

fn main() {
    input!{
        n :isize
    }
    let mut all = 1;
    let mut cnt_exclude_one = 1;
    let mut cnt_exclude_two = 1;
    for i in 0..n {
        all *= 10;
        cnt_exclude_one *= 9;
        cnt_exclude_two *= 8;
        all %= MOD;
        cnt_exclude_one %= MOD;
        cnt_exclude_two %= MOD;
    }
    let ans : isize = (all + cnt_exclude_two - (cnt_exclude_one * 2) % MOD) % MOD;
    println!("{}",(ans+MOD)%MOD);
}
