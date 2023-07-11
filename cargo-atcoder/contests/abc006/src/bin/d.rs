use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        c: [i64; n],
    };

    let mut dp = vec![];
    for &c_i in c.iter() {
        match dp.iter().last() {
            None => dp.push(c_i),
            Some(&max_c) => {
                if max_c < c_i {
                    dp.push(c_i);
                } else {
                    let j = dp.lower_bound(&c_i);
                    dp[j] = c_i;
                }
            }
        }
    }
    let ans = n - dp.len();
    println!("{}", ans);
}
