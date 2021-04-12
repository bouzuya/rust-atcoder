use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyh: [(i64, i64, i64); n],
    };
    let (x, y, h) = {
        let mut p = (0, 0, 0);
        for &(x_i, y_i, h_i) in xyh.iter() {
            if h_i == 0 {
                continue;
            }
            p = (x_i, y_i, h_i);
            break;
        }
        p
    };
    for cx in 0..=100 {
        for cy in 0..=100 {
            let ch = (x - cx).abs() + (y - cy).abs() + h;
            let mut ok = true;
            for &(x_i, y_i, h_i) in xyh.iter() {
                if max(ch - (x_i - cx).abs() - (y_i - cy).abs(), 0) != h_i {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("{} {} {}", cx, cy, ch);
                return;
            }
        }
    }
}
