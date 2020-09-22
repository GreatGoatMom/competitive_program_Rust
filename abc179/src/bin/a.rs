use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input! {
        s : String,
    }
    let slen = s.len();
    if &s[slen-1 .. slen] == "s" {
        println!("{}es",s);
    } else {
        println!("{}s",s);
    }
}
