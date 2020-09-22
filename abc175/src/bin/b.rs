use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        num_list : [i64; n]    
    }
    let mut ans = 0;
    if n < 3 {
        println!("{}",0);
        return;
    }
    for a in 0..(n-2) {
        for b in (a+1)..n-1 {
            for c in (b+1)..n {
                if (num_list[a] == num_list[b] || num_list[b] == num_list[c] || num_list[c] == num_list[a]) {
                    continue;
                }
                let x = num_list[a] + num_list[b];
                let y = num_list[b] + num_list[c];
                let z = num_list[c] + num_list[a];
                if x > num_list[c] && y > num_list[a] && z > num_list[b] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}",ans);
}
