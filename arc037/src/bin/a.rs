use proconio::input;

fn main() {
    input! {
        n: usize,
        m: [i64; n],
    };
    let mut sum = 0_i64;
    for &m_i in m.iter() {
        if m_i < 80 {
            sum += 80 - m_i;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
