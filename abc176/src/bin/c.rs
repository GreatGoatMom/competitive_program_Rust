use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        mut list : [usize;n]
    }
    let mut ans = 0;
    for i in 1..n {
        if list[i-1] > list[i] {
            let dif = list[i-1] - list[i];
            ans += dif;
            list[i] += dif;
        }
    }
    println!("{}",ans);
}
