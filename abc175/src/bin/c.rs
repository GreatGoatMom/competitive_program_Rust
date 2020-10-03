use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        mut x : i128,
        k : i128,
        d : i128
    }
    x = x.abs();
    let mut ans = 0;
    if x - k * d > 0 {
        ans = x - k * d;
    } else {
        let just_before = x / d;
        if (k - just_before) % 2 == 0 {
            ans = x % d;
        } else {
            ans = (x - (d * just_before)) - d;
        }
    }
    println!("{}",ans.abs());
}
