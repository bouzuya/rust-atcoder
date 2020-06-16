// http://shifth.hatenablog.com/entry/2015/05/10/115226
// https://atcoder.jp/contests/tdpc/submissions/6607507
use proconio::input;

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
        a_len: usize,
        b_len: usize,
        mut a: [i64; a_len],
        mut b: [i64; b_len],
    };
    a.reverse();
    b.reverse();

    let ca = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();
    let cb = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &b_i| {
            *acc += b_i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];
    for i_a in 0..=a.len() {
        for i_b in 0..=b.len() {
            let c = ca[i_a] + cb[i_b] - dp[i_a][i_b];
            if i_a + 1 <= a.len() {
                chmax!(dp[i_a + 1][i_b], c + a[i_a]);
            }
            if i_b + 1 <= b.len() {
                chmax!(dp[i_a][i_b + 1], c + b[i_b]);
            }
        }
    }
    let ans = dp[a_len][b_len];
    println!("{}", ans);
}
