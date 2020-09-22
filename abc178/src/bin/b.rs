use proconio::*;
use std::*;

#[fastout]
fn main() {
    input!{
        a : i128,
        b : i128,
        c : i128,
        d : i128
    }
    let mut ans = a*c;
    ans = cmp::max(ans,a*d);
    ans = cmp::max(ans, b*c);
    ans = cmp::max(ans, b*d);
    println!("{}",ans);
}
