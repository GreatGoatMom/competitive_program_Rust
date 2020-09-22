use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

fn main() {
    input! {
        n : i128
    }
    if n <= 1 {
        println!{"{}",0};
    } else if n == 2 {
        println!{"{}",2};
    } else {
        let mut ans = 1;
        ans = ans * (n-2) * 9 * 2 % MOD;
        ans = ans * (n * (n-1) % MOD / 2) % MOD;
        println!{"{}",ans%MOD};
    }
}
