use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;

fn lower_bound(list:&Vec<usize>,target:usize) -> usize{
    let mut right = list.len()-1;
    let mut left = 0;
    let mut middle = (right + left) / 2; 
    while right - left > 1 {
        if list[middle] > target {
            right = middle;
        } else if list[middle] < target {
            left = middle;
        } else if list[middle] == target {
            return middle;
        }
        middle = (right + left) / 2;
    }
    // println!("{} {} {}",list.len(),right,left);

    if list[right] > target {
        left
    } else {
        right
    }
}


#[fastout]
fn main() {
    input!{
        n : usize,
        m : usize,
        k : usize,
        n_list : [usize;n],
        m_list : [usize;m]
    }
    let mut n_sum = vec![0;n+1];
    let mut m_sum = vec![0;m+1];
    for n_i in 0..n {
        n_sum[n_i+1] = n_sum[n_i] + n_list[n_i];
    }
    for m_i in 0..m {
        m_sum[m_i+1] = m_sum[m_i] + m_list[m_i];
    }
    let mut ans = 0;
    for n_i in 0..n+1 {
        if k < n_sum[n_i] {
            break;
        }
        let left = k - n_sum[n_i];
        let cnt = n_i + lower_bound(&m_sum,left);
        ans = cmp::max(ans,cnt);
    }
    for m_i in 0..m+1 {
        if k < m_sum[m_i] {
            break;
        }
        let left = k - m_sum[m_i];
        let cnt = m_i + lower_bound(&n_sum,left);
        ans = cmp::max(ans,cnt);
    }
    println!("{}",ans);

}
