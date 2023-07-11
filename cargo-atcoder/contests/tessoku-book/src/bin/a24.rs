use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut dp = vec![];
    for a_i in a.iter() {
        let j = dp.lower_bound(&a_i);
        if dp.len() == j {
            dp.push(a_i);
        } else {
            dp[j] = a_i;
        }
    }
    let ans = dp.len();
    println!("{}", ans);
}
