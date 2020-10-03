use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        mut n : usize
    }
    let mut ans = Vec::new();
    while n != 0 {
        n-=1;
        ans.push(('a' as u8 + (n%26) as u8) as char);
        n/=26;
    }
    ans.reverse();
    println!("{}",ans.iter().collect::<String>());
}
