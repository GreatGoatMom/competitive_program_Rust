use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        mut x : i128,
        mut k : i128,
        mut d : i128
    }
    x = x.abs();
    k = k.abs();
    d = d.abs();
    let mut ans = 0;
    if x > k * d {
        ans = x - (k * d);
    } else {
        let left = x / d ;
        if (k - left) % 2 == 0 {
            ans = x - (d * left);
        } else {
            ans = d - x % d;
        }
    }
    println!("{}",ans);
}
