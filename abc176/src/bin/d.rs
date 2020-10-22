use proconio::*;
use proconio::marker::{Bytes, Chars};
use std::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        h : usize,
        w : usize,
        mut sx : usize,
        mut sy : usize,
        mut ex : usize,
        mut ey : usize,
        list : [Chars;h]
    }
    let mut ans = 0;
    let mut is_reached = false;
    let mut heapq = BinaryHeap::new();
    let mut reached = vec![vec![1000_000_00;w];h];
    sx -= 1;
    sy -= 1;
    ex -= 1;
    ey -= 1;
    reached[sx][sy] = 0;
    let arrayx = [-1,0,1,0];
    let arrayy = [0,-1,0,1];
    heapq.push((0,[sx,sy]));
    while !heapq.is_empty() {
        let tmp_next = heapq.pop().unwrap();
        let next = tmp_next;
        let times = next.0;
        let tmp_x = next.1[0];
        let tmp_y = next.1[1];
        if tmp_x == ex && tmp_y == ey{
            ans = times;
            is_reached = true;
            break;
        }
        for i in 0..4 {
            if (tmp_x as isize + arrayx[i]) >= 0 && (tmp_x as isize + arrayx[i]) < h as isize{
                if (tmp_y as isize + arrayy[i]) >= 0 && (tmp_y as isize + arrayy[i]) < w as isize{
                    let tx = (tmp_x as isize + arrayx[i]) as usize;
                    let ty = (tmp_y as isize + arrayy[i]) as usize;
                    if (list[tx][ty] == '.' && reached[tx][ty] > -times){
                        heapq.push((times,[tx,ty]));
                        reached[tx][ty] = -times;
                    }
                }
            }
        }
        for i in -2..=2 {
            if (tmp_x as isize + i as isize) < 0 || (tmp_x as isize + i as isize) >= h as isize {
                continue;
            }
            for j in -2..=2 {
                if (tmp_y as isize + j as isize) < 0 || (tmp_y as isize + j as isize) >= w as isize{
                    continue;
                }
                let tx = (tmp_x as isize + i) as usize;
                let ty = (tmp_y as isize + j) as usize;
                if reached[tx][ty] > -(times-1) {
                    if list[tx][ty] == '.' {
                        heapq.push((times-1,[tx,ty]));
                    }
                    reached[tx][ty] = -(times-1);
                }
            }
        }
    }
    ans = if is_reached {
        -ans 
    } else {
        -1
    };
    println!{"{}",ans};
}
