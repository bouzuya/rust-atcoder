use std::collections::HashSet;

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
        xy: [(i64, i64); n],
    };
    let mut set1 = HashSet::new();
    let mut set = HashSet::new();
    for (i, (x_i, y_i)) in xy.iter().copied().enumerate() {
        for (j, (x_j, y_j)) in xy.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            if x_i == x_j {
                set1.insert(if y_j - y_i > 0 { 1 } else { -1 });
            } else {
                let dy = y_j - y_i;
                let dx = x_j - x_i;
                let g = gcd(dx, dy);
                set.insert((dy / g, dx / g));
            }
        }
    }
    let ans = set.len() * 2 + set1.len();
    println!("{}", ans);
}
