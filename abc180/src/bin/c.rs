use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize
    }
    let mut ans = vec![];
    let limit : usize = ((n as f64).sqrt()) as usize;
    for i in 1..=limit {
        if n % i == 0 {
            ans.push(i);
            if n / i != i {
                ans.push(n/i);
            }
        }
    }
    ans.sort();
    for i in ans.iter(){
        println!("{}",i);
    }
}
