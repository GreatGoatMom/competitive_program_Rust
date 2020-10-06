use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

fn score(vec: &Vec<usize>,list: &Vec<Vec<usize>>,q :usize) -> usize{
    let mut res = 0;
    for ind in 0..q {
        if vec[list[ind][1]] - vec[list[ind][0]] == list[ind][2] {
            res += list[ind][3];
        }
    }
    res
}

fn dfs(vec : &mut Vec<usize>,n : usize,m : usize,list : &Vec<Vec<usize>>, q : usize) -> usize{

    if vec.len() == n {
        return score(vec,list,q);
    }

    let pre_num = match vec.last() {
        Some(i) => *i,
        None => 1
    };
    let mut res = 0;
    for i in pre_num..=m {
        vec.push(i);
        res = cmp::max(res,dfs(vec,n,m,list,q));
        vec.pop();
    }
    res
}


fn main() {
    input!{
        n : usize,
        m : usize, 
        q : usize, 
        mut list : [[usize;4];q]
    }
    for l_ind in 0..q {
        list[l_ind][0] -= 1;
        list[l_ind][1] -= 1;
    }
    let mut origin_list = vec![];
    let ans = dfs(&mut origin_list,n,m,&list,q);
    println!("{}",ans);
}
