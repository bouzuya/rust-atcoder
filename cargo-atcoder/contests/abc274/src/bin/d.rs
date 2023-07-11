use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n],
    };
    let sx = a[0];
    let sy = 0;

    let mut xs = vec![];
    for i in (2..n).step_by(2) {
        xs.push(a[i]);
    }

    let mut dp = HashSet::new();
    dp.insert(sx);
    for xs_i in xs.iter().copied() {
        let mut next = HashSet::new();
        for d in dp.iter().copied() {
            next.insert(d + xs_i);
            next.insert(d - xs_i);
        }
        dp = next;
    }
    if !dp.contains(&x) {
        println!("No");
        return;
    }

    let mut ys = vec![];
    for i in (1..n).step_by(2) {
        ys.push(a[i]);
    }
    let mut dp = HashSet::new();
    dp.insert(sy);
    for ys_i in ys.iter().copied() {
        let mut next = HashSet::new();
        for d in dp.iter().copied() {
            next.insert(d + ys_i);
            next.insert(d - ys_i);
        }
        dp = next;
    }
    if !dp.contains(&y) {
        println!("No");
        return;
    }

    println!("Yes");
}
