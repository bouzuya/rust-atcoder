// Original: https://atcoder.jp/contests/abc162/submissions/11852555
use proconio::input;

fn main() {
    input! {
        n :usize,
        av: [i64; n],
    };

    let ans = if n % 2 == 0 {
        let mut dp = (0_i64, 0_i64);
        for (i, &a) in av.iter().enumerate() {
            if i % 2 == 0 {
                dp.0 = dp.0 + a;
            } else {
                dp.1 = std::cmp::max(dp.0, dp.1 + a);
            }
        }
        dp.1
    } else {
        let mut dp = (0_i64, 0_i64, 0_i64);
        for (i, &a) in av.iter().enumerate() {
            if i % 2 == 0 {
                dp.0 = dp.0 + a;
                if i != 0 {
                    dp.2 = std::cmp::max(dp.1, dp.2 + a);
                }
            } else {
                dp.1 = std::cmp::max(dp.0, dp.1 + a);
            }
        }
        dp.2
    };
    println!("{}", ans);
}
