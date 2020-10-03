use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

fn main() {
    input!{
        x : usize,
        n : usize,
        list : [usize;n]
    }
    let mut forbidden = vec![false;102];
    for i in list {
        forbidden[i] = true;
    }

    let mut ans = 101;
    for i in x..=101 {
        if !forbidden[i] {
            ans = i;
            break;
        }
    }
    for j in (0..=x).rev() {
        if !forbidden[j] {
            let pre : isize = (x as isize - ans as isize).abs();
            let now : isize = (x as isize  - j as isize).abs();
            if pre >= now {
                ans = j;
            }
            break;
        }
    }

    println!("{}",ans);
}
