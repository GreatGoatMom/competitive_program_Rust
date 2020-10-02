use proconio::*;
use std::*;

const MOD: i128 = 1_000_000_007;

fn main() {
    input! {
        n : i128
    }
    if n <= 1 {
        println!{"{}",0};
    } else if n == 2 {
        println!{"{}",2};
    } else {
        let mut all=1i128;
        let mut nine=1i128;
        let mut eight=1i128;
        for _ in 0..n {
            all = all * 10 % MOD;
            nine = nine * 9 % MOD;
            eight = eight * 8 % MOD;
        }
        let mut ans : i128 = (all - (nine * 2) % MOD + eight) % MOD;
        println!{"{}",(ans+MOD)%MOD};
    }
}
