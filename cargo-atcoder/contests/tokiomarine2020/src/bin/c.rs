use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };

    let mut b = a.clone();
    for _ in 0..k {
        let mut c = vec![0; n + 1];
        for (i, &b_i) in b.iter().enumerate() {
            c[i.saturating_sub(b_i as usize)] += 1;
            if c[i.saturating_sub(b_i as usize)] > n as i64 {
                c[i.saturating_sub(b_i as usize)] = n as i64;
            }
            c[cmp::min(i + b_i as usize + 1, n)] -= 1;
        }
        for i in 0..n {
            c[i + 1] += c[i];
        }
        b = c;
        if (0..n).all(|i| b[i] == n as i64) {
            break;
        }
    }

    for (i, &b_i) in b.iter().enumerate().take(n) {
        print!("{}{}", b_i, if i == n - 1 { "\n" } else { " " });
    }
}
