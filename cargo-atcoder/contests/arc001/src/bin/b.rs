use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let mut min = 40 + 40 + 40;
    for x in 0..=40 {
        for y in 0..=40 {
            for z in 0..=40 {
                if (a + x * 1 + y * 5 + z * 10 == b)
                    || (a + x * 1 + y * 5 - z * 10 == b)
                    || (a + x * 1 - y * 5 + z * 10 == b)
                    || (a + x * 1 - y * 5 - z * 10 == b)
                    || (a - x * 1 + y * 5 + z * 10 == b)
                    || (a - x * 1 + y * 5 - z * 10 == b)
                    || (a - x * 1 - y * 5 + z * 10 == b)
                    || (a - x * 1 - y * 5 - z * 10 == b)
                {
                    min = cmp::min(min, x + y + z);
                }
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
