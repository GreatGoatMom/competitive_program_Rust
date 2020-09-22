use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input! {
        a : i32,
        b : i32,
        c : i32
    }
    if b * c >= a {
        println!{"Yes"};
    } else {
        println!{"No"};
    }
}
