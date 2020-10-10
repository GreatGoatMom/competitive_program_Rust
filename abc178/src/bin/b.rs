use proconio::*;
use std::*;

#[fastout]
fn main() {
    input!{
        a : isize,
        b : isize,
        c : isize,
        d : isize
    }
    let vec = vec![a*c,a*d,b*c,b*d];
    let ans = vec.iter().max().unwrap();
    println!("{}",ans);
}
