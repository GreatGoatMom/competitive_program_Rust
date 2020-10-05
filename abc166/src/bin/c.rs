use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        m : usize,
        h_list : [usize;n],
        tied_list : [(usize,usize);m],
    }
    let mut related_list = vec![vec![];n];

    for l in tied_list {
        let a = l.0 - 1;
        let b = l.1 - 1;
        related_list[a].push(b);
        related_list[b].push(a);        
    }

    let mut ans = 0;
    for l_i in 0..n {
        let high = h_list[l_i];
        let mut is_highest = true;
        for l_l in &related_list[l_i] {
            if high <= h_list[*l_l] {
                is_highest = false;
            }
        }
        if is_highest {
            ans += 1;
        }
    }
    println!("{}",ans);
}
