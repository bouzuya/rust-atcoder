use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };
    let mut s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();
    s.sort();

    let mut ng = -1_i64;
    let mut ok = 1 << 60;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        let mut count = 0;
        for (i, s_i) in s.iter().copied().enumerate() {
            count += s[i + 1..].upper_bound(&(s_i + x));
        }
        if count >= k {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
