use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: Usize1,
        mut h: [i64; n],
    };
    h.sort();
    let mut ans = 1_000_000_000_i64;
    for i in k..n {
        let h_max = h[i];
        let h_min = h[i - k];
        ans = std::cmp::min(ans, h_max - h_min);
    }
    println!("{}", ans);
}
