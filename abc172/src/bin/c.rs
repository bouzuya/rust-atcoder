use proconio::input;
use superslice::*;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n],
        b: [i64; m],
    };
    let ca = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let cb = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, b_i| {
            *acc += b_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..=n {
        let x = k - ca[i];
        if x < 0 {
            continue;
        }
        let j = cb.upper_bound(&x);
        chmax!(ans, i + j - 1);
    }
    println!("{}", ans);
}
