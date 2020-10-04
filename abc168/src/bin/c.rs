use proconio::*;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use std::*;

#[allow(dead_code)]
const MOD: i128 = 100_000_007;

#[fastout]
fn main() {
    input!{
        a : f64,
        b : f64,
        h : usize,
        m : usize
    }
    let h_deg = (h * 30) as f64 + 0.5 * m as f64;
    let m_deg = (m * 6) as f64;
    let mut deg = if h_deg > m_deg {
        h_deg - m_deg
    } else {
        m_deg - h_deg
    };
    if deg > 180. {
        deg = 360. - deg;
    }
    let ans = a * a + b * b - 2. * a * b * ((deg as f64).to_radians()).cos();
    println!("{}",ans.sqrt());
}
