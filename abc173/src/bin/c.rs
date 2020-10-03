use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;

#[warn(unused_imports)]
const MOD: i128 = 100_000_007;


fn count_black_block(grid:&Vec<Vec<char>>,h:usize,w:usize,rev:bool) -> usize {
    let mut cnt = 0;
    for h_i in 0..h {
        for w_j in 0..w {
            let mut hh = h_i;
            let mut ww = w_j;
            if rev {
                hh = w_j;
                ww = h_i;
            }
            cnt += match grid[hh][ww] {
                '#' => 1,
                _ => 0
            };
        }
    }
    cnt
}


fn main() {
    input!{
        h : usize,
        w : usize,
        k : usize,
        grid : [Chars; h] 
    }
    let mut h_cnt_list = vec![0;h+1];
    let mut w_cnt_list = vec![0;w+1];
    let mut all_cnt = 0;
    for h_i in 0..h {
        for w_j in 0..w {
            if grid[h_i][w_j] == '#' {
                h_cnt_list[h_i] += 1;
                w_cnt_list[w_j] += 1;
                all_cnt += 1;
            }
        }
    }
    let mut ans = 0;
    for h_i in 0..(1<<h) {
        for w_i in 0..(1<<w){
            let mut cnt = 0;
            for h_b in 0..h {
                for w_b in 0..w {
                    if h_i & (1<<h_b) == 0 && w_i & (1<<w_b) == 0{
                        if grid[h_b][w_b] == '#' {
                            cnt+= 1;
                        }
                    }
                }
            }
            if cnt == k {
                ans += 1;
            }
        }
    }
    println!("{}",ans);

}
