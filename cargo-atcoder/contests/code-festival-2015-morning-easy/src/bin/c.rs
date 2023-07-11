use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: i64,
        r: i64,
        mut s: [i64; n - 1],
    };
    let mut sum = r * k as i64;
    if k == n {
        let x = s.iter().sum::<i64>();
        let ans = if x >= sum { 0 } else { sum - x };
        let ans = if ans > m { -1 } else { ans };
        println!("{}", ans);
    } else {
        s.sort_by_key(|&s_i| cmp::Reverse(s_i));
        let x1 = s[0..k - 1].iter().sum::<i64>();
        let xa = s[0..k].iter().sum::<i64>();
        let ans = if xa >= sum {
            0
        } else {
            if x1 >= sum {
                0
            } else {
                sum - x1
            }
        };
        let ans = if ans > m { -1 } else { ans };
        println!("{}", ans);
    }
}
