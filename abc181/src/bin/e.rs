use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i64; n],
        w: [i64; m],
    };
    h.sort();
    let c1 = std::iter::once(0)
        .chain((0..n).take(n - 1).step_by(2).scan(0, |acc, i| {
            *acc += (h[i] - h[i + 1]).abs();
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let c2 = {
        h.reverse();
        let mut c2 = std::iter::once(0)
            .chain((0..n).take(n - 1).step_by(2).scan(0, |acc, i| {
                *acc += (h[i] - h[i + 1]).abs();
                Some(*acc)
            }))
            .collect::<Vec<i64>>();
        h.reverse();
        c2.reverse();
        c2
    };
    let mut ans = 1_000_000_000_000_000;
    for w_i in w {
        let j = h.lower_bound(&w_i);
        let a = c1[j / 2] + c2[j / 2] + (h[j - if j % 2 != 0 { 1 } else { 0 }] - w_i).abs();
        ans = std::cmp::min(ans, a);
    }
    println!("{}", ans);
}
