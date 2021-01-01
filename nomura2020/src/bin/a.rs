use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        h_1: i64,
        m_1: i64,
        h_2: i64,
        m_2: i64,
        k: i64,
    };
    let m_1 = h_1 * 60 + m_1;
    let m_2 = h_2 * 60 + m_2;
    let ans = max(0, m_2 - m_1 - k);
    println!("{}", ans);
}
