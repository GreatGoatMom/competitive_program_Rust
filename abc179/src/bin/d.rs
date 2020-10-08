use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        k : usize
    }
    let mut num_list = [0;20_0002];
    for _ in 0..k {
        input!{
            a : usize,
            b : usize
        }
        num_list[a] += 1;
        num_list[b+1] -= 1; 
    }
    for ind in 1..=200_000 {
        num_list[ind] += num_list[ind-1];
    }
    let mut dp = [0;20_0001];
    dp[1] = 1;
    for i in 0..20_000 {
        for j in &num_list {
            if i-j >= 0 {
                dp[i-j] += dp[i];
            }
        }
    }
    let ans = dp[n+1] % 998244353;
    println!("{}",ans);
}
