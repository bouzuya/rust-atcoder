use std::collections::BTreeSet;

use proconio::input;

fn gcd(n: i64, m: i64) -> i64 {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut set = vec![BTreeSet::new(); n];
    set[0].insert(ab[0]);
    for (i, (a_i, b_i)) in ab.iter().copied().enumerate().skip(1) {
        for (a_p, b_p) in set[i - 1].clone() {
            set[i].insert((gcd(a_p, a_i), gcd(b_p, b_i)));
            set[i].insert((gcd(a_p, b_i), gcd(b_p, a_i)));
        }
    }
    let ans = set[n - 1]
        .iter()
        .copied()
        .map(|(a, b)| a * b / gcd(a, b))
        .max()
        .unwrap();
    println!("{}", ans);
}
