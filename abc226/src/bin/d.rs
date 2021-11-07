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
    let mut set = HashSet::new();
    for (i, (x1, y1)) in xy.iter().copied().enumerate() {
        for (j, (x2, y2)) in xy.iter().copied().enumerate() {
            if i == j {
                continue;
            }

            let y = y2 - y1;
            let x = x2 - x1;
            if x == 0 {
                set.insert((y.signum(), x.signum(), y.signum(), x));
            } else if y == 0 {
                set.insert((y.signum(), x.signum(), y, x.signum()));
            } else {
                let g = gcd(y, x);
                set.insert((y.signum(), x.signum(), y / g, x / g));
            }
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
