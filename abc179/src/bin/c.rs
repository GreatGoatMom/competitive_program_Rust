use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : i64
    }
    let mut ans = 0;
    let limit = n + 1;
    for i in 1..limit {
        let dev = if n % i == 0 {
            n / i - 1
        } else {
            n / i 
        };
        ans += dev;
    }
    println!("{}",ans);
}
