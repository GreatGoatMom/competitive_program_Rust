use proconio::*;
use std::*;

const MOD: usize = 1000_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        list : [usize;n]
    }
    let mut ans = 0;
    let mut sum = 0;
    for i in 1..n {
        sum += list[i];
        sum %= MOD;
    }
    for i in 0..n-1 {
        let tmp = (sum * list[i]) % MOD;
        ans += tmp;
        ans %= MOD;
        sum -= list[i+1];
        sum = (sum + MOD) % MOD;
    }
    println!("{}",ans%MOD);
}
