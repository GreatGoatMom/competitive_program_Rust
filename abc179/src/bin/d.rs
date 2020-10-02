use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        k : usize
    }
    let mut num_list = Vec::new();
    for _ in 0..k {
        input!{
            a : usize,
            b : usize
        }
        num_list.push(a);
        num_list.push(b);
    }
    num_list.sort();
    let mut dp = [1;20_0001];
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
