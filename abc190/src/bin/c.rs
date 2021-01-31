use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    };
    let mut max_count = 0;
    for bits in 0..1 << k {
        let mut s = vec![false; n];
        for (i, &(c_i, d_i)) in cd.iter().enumerate() {
            s[if (bits >> i) & 1 == 1 { c_i } else { d_i }] = true;
        }
        let count = ab.iter().filter(|&&(a_i, b_i)| s[a_i] && s[b_i]).count();
        max_count = max(max_count, count);
    }
    let ans = max_count;
    println!("{}", ans);
}
