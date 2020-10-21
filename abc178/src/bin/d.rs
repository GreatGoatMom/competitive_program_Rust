use proconio::*;
use std::*;


const MOD : usize = 1_000_000_007;

fn main() {
    input!{
        s : usize
    }
    let mut dp = vec![0;s+1];
    dp[0] = 1;
    for i in 1..=s{
        if i < 3 {
            continue;
        }
        for j in 0..=i-3 {
            dp[i] += dp[j];
            dp[i] %= MOD;
        }
    }
    println!{"{}",dp[s]};
}
