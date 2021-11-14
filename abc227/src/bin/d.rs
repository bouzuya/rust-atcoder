use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ok = 0;
    let mut ng = std::usize::MAX;
    while ng - ok > 1 {
        let p = (ng + ok) / 2;
        let sum = a.iter().copied().map(|a_i| cmp::min(a_i, p)).sum::<usize>();
        if match p.checked_mul(k) {
            Some(pk) => sum >= pk,
            None => false,
        } {
            ok = p;
        } else {
            ng = p;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
