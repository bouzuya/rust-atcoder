use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    };
    let mut dp = vec![];
    for (i, a_i) in a.into_iter().rev().enumerate() {
        if !(i..=(p - (n - 1 - i))).contains(&a_i) {
            continue;
        }
        let b_i = a_i - i;
        let index = dp.upper_bound(&b_i);
        if index < dp.len() {
            dp[index] = b_i;
        } else {
            dp.push(b_i);
        }
    }
    let ans = n - dp.len();
    println!("{}", ans);
}
