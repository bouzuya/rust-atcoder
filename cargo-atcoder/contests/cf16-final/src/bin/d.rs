use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
    };

    let mut c = HashMap::new();
    let mut r = vec![0_usize; m];
    for x_i in x.iter().copied() {
        r[x_i % m] += 1;
        *c.entry(x_i).or_insert(0) += 1_usize;
    }

    let mut s = vec![0_usize; m];
    for (x_i, count) in c {
        s[x_i % m] += count / 2 * 2;
    }

    let mut ans = 0_usize;
    for i in 0..=m / 2 {
        let j = (m - i) % m;
        if i == j {
            let a = r[i] / 2;
            ans += a;
            r[i] -= a * 2;
            continue;
        }
        let (i, j) = if r[i] < r[j] { (i, j) } else { (j, i) };
        ans += r[i] + s[j].min(r[j] - r[i]) / 2;
    }

    println!("{}", ans);
}
