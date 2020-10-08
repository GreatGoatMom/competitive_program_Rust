use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        list : [(usize,usize); n]
    }
    let mut cnt = 0;
    let mut serial_cnt = false;
    for l in list {
        if l.0 == l.1  {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt >= 3 {
            serial_cnt = true;
        }
    }
    let ans = if serial_cnt {
        "Yes"
    } else {
        "No"
    };
    println!(
        "{}",
        ans
    );
}
