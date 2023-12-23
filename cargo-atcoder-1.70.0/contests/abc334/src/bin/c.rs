use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };
    let set = a.iter().copied().collect::<HashSet<usize>>();

    let mut b = vec![];
    for i in 1..=n {
        b.push(i);
        if !set.contains(&i) {
            b.push(i);
        }
    }

    if (2 * n - k) % 2 == 0 {
        let mut ans = 0_usize;
        for i in (0..b.len()).step_by(2) {
            ans += b[i + 1] - b[i];
        }
        println!("{}", ans);
    } else {
        let mut c0 = vec![0];
        let mut c1 = vec![0];
        for i in (0..b.len() - 1).step_by(2) {
            c0.push(b[i + 1] - b[i]);
        }
        for i in (1..b.len()).rev().step_by(2) {
            c1.push(b[i] - b[i - 1]);
        }

        for i in 1..c0.len() {
            c0[i] += c0[i - 1];
        }
        for i in 1..c1.len() {
            c1[i] += c1[i - 1];
        }
        c1.reverse();
        let ans = c0
            .into_iter()
            .zip(c1.into_iter())
            .map(|(x, y)| x + y)
            .min()
            .unwrap();
        println!("{}", ans);
    }
}
