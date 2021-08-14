use std::cmp;

use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        s: [i64; n],
        t: [i64; n],
    };
    let s = s.iter().chain(s.iter()).copied().collect::<Vec<i64>>();
    let t = t.iter().chain(t.iter()).copied().collect::<Vec<i64>>();
    let inf = 1_000_000_000;
    let mut m = vec![inf; n];
    let mut c = t[0];
    for i in 0..2 * n {
        c = cmp::min(t[i], c);
        chmin!(m[i % n], c);
        c += s[i];
    }
    for m_i in m {
        println!("{}", m_i);
    }
}
