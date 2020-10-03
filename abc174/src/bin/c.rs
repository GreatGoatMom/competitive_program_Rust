use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

fn main() {
    input!{
        x : usize
    }
    let mut vec : Vec<bool> = vec![false;x+1];
    let mut cnt = 0;
    let mut ten_cnt = 10;
    let mut div = 0;
    loop {
        cnt += 1;
        div = (7 + div * ten_cnt) % x;
        if div == 0 {
            println!("{}",cnt);
            process::exit(0);
        }
        if vec[div]{
            break;
        }
        vec[div] = true;
    }
    println!("-1");
}
