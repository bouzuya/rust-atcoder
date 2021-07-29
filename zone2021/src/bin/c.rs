use std::collections::HashSet;

use proconio::input;

fn f(abcde: &[(usize, usize, usize, usize, usize)], x: usize) -> bool {
    let set = abcde
        .iter()
        .map(|&(a, b, c, d, e)| {
            (if a >= x { 1 } else { 0 } << 4)
                | (if b >= x { 1 } else { 0 } << 3)
                | (if c >= x { 1 } else { 0 } << 2)
                | (if d >= x { 1 } else { 0 } << 1)
                | (if e >= x { 1 } else { 0 })
        })
        .collect::<HashSet<usize>>();
    for m1 in set.iter().copied() {
        for m2 in set.iter().copied() {
            for m3 in set.iter().copied() {
                if m1 | m2 | m3 == 0b11111 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        abcde: [(usize, usize, usize, usize, usize); n],
    };
    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        if f(&abcde, x) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
