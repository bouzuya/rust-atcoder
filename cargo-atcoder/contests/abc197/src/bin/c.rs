use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut ans = 0_u64;
    for &a_i in a.iter() {
        ans |= a_i;
    }
    for bits in 0..1 << (n - 1) {
        let mut is = vec![];
        for i in 0..(n - 1) {
            is.push((bits >> i) & 1 == 1);
        }

        let mut ors: Vec<u64> = vec![];
        ors.push(a[0]);
        for i in 1..n {
            if is[i - 1] {
                ors.push(a[i]);
            } else {
                let j = ors.len() - 1;
                ors[j] |= a[i];
            }
        }
        let mut xor = ors[0];
        for &or_i in ors.iter().skip(1) {
            xor ^= or_i;
        }
        ans = min(ans, xor);
    }

    println!("{}", ans);
}
