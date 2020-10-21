use proconio::*;
use std::*;

const MOD : usize = 998244353;
fn main() {
    input!{
        n : usize,
        k : usize,
        list : [[usize;2];k]
    }
    let mut dp = vec![0;n+1];
    let mut sum_dp = vec![0;n+1];
    dp[0] = 1;
    sum_dp[1] = 1;
    for i in 1..n {
        for ik in 0..k {
            let right = if i + 1< list[ik][0] {
                0
            } else {
                i + 1 - list[ik][0]
            };
            let left = if i < list[ik][1] {
                0
            } else {
                i - list[ik][1]
            }; 
            dp[i] += (sum_dp[right]  + MOD - sum_dp[left]) % MOD;
            dp[i] %= MOD;
        }
        sum_dp[i+1] += (sum_dp[i] + dp[i]) %MOD;
    }
    println!{"{}",dp[n-1]%MOD};
}
