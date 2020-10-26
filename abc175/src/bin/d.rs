use proconio::*;
use std::*;

const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        n : usize,
        k : usize,
        p : [usize;n],
        c : [isize;n]
    }
    let all_cnt : isize = c.iter().fold(0,|ac, i| ac + i);
    let mut ans : isize = -100_000_000;
    let is_loop = n < k;
    let loop_cnt = match is_loop {
        true => k % n,
        false => k
    };
    for i in 0..n {
        let mut tmp_cnt = 0;
        let mut tmp_sum : isize = 0;
        let next_ind : usize = i;
        while tmp_cnt < loop_cnt {
            let next_ind = p[next_ind] - 1;
            tmp_sum += c[next_ind];
            tmp_cnt += 1;
        }
        if is_loop {
            tmp_sum += all_cnt * (k / n) as isize;
        }
        ans = cmp::max(ans,tmp_sum);
    }
    println!{"{}",ans};
}
