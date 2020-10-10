use proconio::*;
use std::*;

const MOD: i128 = 998244353;

fn main() {
    input!{
        n : usize,
        k : usize,
        list : [[usize;2];k]
    }
    let mut dp = vec![0;n+1];
    let mut s_dp = vec![0;n+1];
    dp[0] = 1;
    s_dp[1] = 1;
    for i in 1..n {
        for kind in 0..k {
            let left = if i <= list[kind][1]{
                0
            } else {
                i - list[kind][1]
            };
            let right =  if i + 1 <= list[kind][0] {
                0
            } else {
                i - list[kind][0] + 1
            };
            dp[i] += (s_dp[right] - s_dp[left] + MOD) % MOD;
        }
        s_dp[i+1] += s_dp[i]+dp[i];
        s_dp[i+1] %= MOD;
    }
    println!("{}",dp[n-1]%MOD);
}
