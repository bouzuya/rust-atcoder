use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrx: [(Usize1, Usize1, usize); m],
    };
    let mut max_bit_count = -1;
    for bits in 0..1 << n {
        let mut bit_count = 0;
        let mut c = vec![0; n + 1];
        for i in 0..n {
            let b = (bits >> i) & 1 == 1;
            c[i + 1] = c[i] + if b { 1 } else { 0 };
            if b {
                bit_count += 1;
            }
        }
        let mut ok = true;
        for &(l_i, r_i, x_i) in lrx.iter() {
            if c[r_i + 1] - c[l_i] != x_i {
                ok = false;
            }
        }
        if ok {
            max_bit_count = max(max_bit_count, bit_count);
        }
    }
    let ans = max_bit_count;
    println!("{}", ans);
}
