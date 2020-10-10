use proconio::*;

#[fastout]
fn main() {
    input!{
        n : isize
    }
    let mut ans = n - 1;
    println!("{}",ans.abs());
}
