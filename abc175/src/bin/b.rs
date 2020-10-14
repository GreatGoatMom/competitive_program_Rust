use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        list : [usize;n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i+1)..n {
            for k in (j+1)..n {
                if (list[i] == list[j] || list[j] == list[k] || list[k] == list[i]) {
                    continue;
                }
                let x = list[i] + list[j];
                let y = list[j] + list[k];
                let z = list[k] + list[i];
                if (x > list[k] && y > list[i] && z > list[j]){
                    ans += 1;
                }
            }
        }
    }
    println!("{}",ans);
}
