use std::cmp;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };
    let d = |n: usize| -> usize { n.to_string().len() };
    let mut ok = 0_usize;
    let mut ng = 1_000_000_000_000_usize;
    while ng - ok > 1 {
        let n = (ng + ok) / 2;
        match a
            .checked_mul(n)
            .and_then(|an| b.checked_mul(d(n)).and_then(|bdn| an.checked_add(bdn)))
        {
            Some(v) if v <= x => ok = n,
            Some(_) | None => ng = n,
        }
    }
    let ans = cmp::min(ok, 1_000_000_000);
    println!("{}", ans);
}
