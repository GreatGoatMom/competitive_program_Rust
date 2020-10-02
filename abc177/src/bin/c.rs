use proconio::*;
use std::*;

const MOD: i128 = 1000_000_007;

#[fastout]
fn main() {
    input!{
        n : usize, 
        num_list : [i128;n]
    }
    let mut sum = 0;
    for i in 1..n {
        sum += num_list[i];
        sum %= MOD;
    }
    let mut ans = 0;
    for i  in 0..n-1 {
        ans += (num_list[i] * sum) % MOD;
        ans %= MOD;
        sum -= num_list[i+1];
        sum = (sum + MOD) % MOD;
    }
    println!{"{}",ans};
}
