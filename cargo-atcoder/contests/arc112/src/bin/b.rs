use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        b: i64,
        c: i64,
    };
    if c == 1 {
        // * -1
        if b == 0 {
            // 0 -> 0
            println!("{}", 1);
        } else {
            // 1 -> -1, 1
            // -1 -> -1, 1
            println!("{}", 2);
        }
        return;
    } else if c == 2 {
        // * -1
        // + -1
        if b == 0 {
            // 0 -> -1, 0
            println!("{}", 2);
        } else {
            // 1 -> -1, 0, 1
            // -1 -> -2, -1, 1
            println!("{}", 3);
        }
        return;
    } else {
        if b == 0 {
            let min_n = -c / 2;
            let max_n = 0;
            let min_p = 0;
            let max_p = (c - 1) / 2;
            let ans = (max_n - min_n + 1) + (max_p - min_p + 1)
                - if max_n == 0 && min_p == 0 { 1 } else { 0 };
            println!("{}", ans);
        } else if b < 0 {
            let min_n = b - c / 2;
            let max_n = if -b - (c - 1) / 2 <= 0 {
                0
            } else {
                max(b, -(-b - (c - 2) / 2))
            };
            let min_p = max(0, -b - (c - 1) / 2);
            let max_p = -(b - (c - 1) / 2);
            let ans = (max_n - min_n + 1) + (max_p - min_p + 1)
                - if max_n == 0 && min_p == 0 { 1 } else { 0 };
            println!("{}", ans);
        } else if b > 0 {
            let min_n = -b - (c - 1) / 2;
            let max_n = if b - (c / 2) <= 0 {
                0
            } else {
                -(b - (c - 1) / 2)
            };
            let min_p = max(0, b - (c / 2));
            let max_p = max(b, max(-(b - (c - 1) / 2), -(-b - (c - 2) / 2)));
            let ans = (max_n - min_n + 1).abs() + (max_p - min_p + 1)
                - if max_n == 0 && min_p == 0 { 1 } else { 0 };
            println!("{}", ans);
        }
    }
}
