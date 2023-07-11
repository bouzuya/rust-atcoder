use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        c: usize,
        nml: [(i64, i64, i64); c],
    };
    let mut max_n = 0;
    let mut max_m = 0;
    let mut max_l = 0;
    for (n_i, m_i, l_i) in nml {
        let mut v = vec![n_i, m_i, l_i];
        v.sort();
        let (n_i, m_i, l_i) = (v[0], v[1], v[2]);
        max_n = max(max_n, n_i);
        max_m = max(max_m, m_i);
        max_l = max(max_l, l_i);
    }
    let ans = max_n * max_m * max_l;
    println!("{}", ans);
}
