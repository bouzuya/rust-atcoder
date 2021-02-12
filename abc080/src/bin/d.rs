use proconio::{input, marker::Usize1};
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        c: usize,
        stc: [(Usize1, Usize1, Usize1); n],
    };
    let mut v_s = vec![0; 200_000 + 2];
    for c_j in 0..c {
        let mut v = vec![0; 200_000 + 2];
        for &(s_i, t_i, c_i) in stc.iter() {
            if c_i == c_j {
                v[2 * s_i + 1] += 1;
                v[2 * t_i + 2] -= 1;
            }
        }
        for i in 0..v.len() - 1 {
            v[i + 1] += v[i];
        }
        for i in 0..v.len() {
            if v[i] > 0 {
                v_s[i] += 1;
            }
        }
    }
    let mut max_v = 0;
    for &v_i in v_s.iter() {
        max_v = max(max_v, v_i);
    }
    let ans = max_v;
    println!("{}", ans);
}
