use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : usize,
        b : usize,
        c : usize
    }
    let ans = if b * c >= a {
        "Yes"
    } else {
        "No"
    };
    println!("{}",ans);
}
