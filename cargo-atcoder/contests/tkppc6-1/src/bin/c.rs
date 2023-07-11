use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 2 * n - 1],
    };
    let a0 = a[0];
    let mut e = 0;
    let mut l = 0;
    let mut g = 0;
    for a_i in a.into_iter().skip(1) {
        if a_i < 0 {
            continue;
        }
        match a_i.cmp(&a0) {
            Ordering::Less => l += 1,
            Ordering::Equal => e += 1,
            Ordering::Greater => g += 1,
        }
    }
    if g > n - 1 || l > n - 1 {
        println!("No");
        return;
    }
    let x = 2 * n - 2 - e;
    println!("{}", if !(g > x || l > x) { "Yes" } else { "No" });
}
