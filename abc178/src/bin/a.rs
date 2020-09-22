use proconio::*;

#[fastout]
fn main() {
    input! {
        n : u32,
    }
    let ans;
    if n == 1 {
        ans = 0;
    } else {
        ans = 1;
    }
    println!{"{}", ans};
}
