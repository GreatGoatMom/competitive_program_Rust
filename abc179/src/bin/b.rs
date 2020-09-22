use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n :i8,
        numList : [(i8,i8); n],
    }
    let mut ans = 0;
    let mut tmp = 0;
    for num in numList {
        if num.0 == num.1 {
            tmp += 1;
        } else {
            ans = cmp::max(ans,tmp);
            tmp = 0;
        }
    }
    ans = cmp::max(ans,tmp);
    let ansStr = if ans >= 3 {
        "Yes"
    } else {
        "No"
    };
    println!("{}",ansStr);
}
