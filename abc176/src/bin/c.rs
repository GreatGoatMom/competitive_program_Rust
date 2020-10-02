use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        n_list: [i64;n]
    }
    let mut ans = 0;
    let mut high = n_list[0];
    for i in 1..n {
        if high > n_list[i] {
            ans += high - n_list[i];
        } else {
            high = n_list[i];
        }
    }
    println!{"{}",ans};

}
