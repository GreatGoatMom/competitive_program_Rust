use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize
    }
    let mut ans = 0;
    for i in 1..n+1{
        let dev = n / i;
        for j in 1..=dev {
            if i * j != n {
                ans += 1;
            }
        }
    }
    println!("{}",ans);
}
