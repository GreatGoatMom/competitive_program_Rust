use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        x : usize,
        t : usize
    }
    let mut ans = n / x;
    if n % x != 0 {
        ans += 1;
    }
    println!("{}",ans*t);
}
