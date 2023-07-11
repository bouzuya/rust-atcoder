// 解説 AC
use proconio::input;

fn is_ok(t: f64, xyc: &Vec<(i64, i64, i64)>) -> bool {
    let (x_0, y_0, c_0) = xyc[0];
    let mut range_x = (x_0 as f64 - t / c_0 as f64, x_0 as f64 + t / c_0 as f64);
    let mut range_y = (y_0 as f64 - t / c_0 as f64, y_0 as f64 + t / c_0 as f64);
    for &(x_i, y_i, c_i) in xyc.iter().skip(1) {
        let r_x = (x_i as f64 - t / c_i as f64, x_i as f64 + t / c_i as f64);
        let r_y = (y_i as f64 - t / c_i as f64, y_i as f64 + t / c_i as f64);
        let x_min = if range_x.0 > r_x.0 { range_x.0 } else { r_x.0 };
        let x_max = if range_x.1 < r_x.1 { range_x.1 } else { r_x.1 };
        let y_min = if range_y.0 > r_y.0 { range_y.0 } else { r_y.0 };
        let y_max = if range_y.1 < r_y.1 { range_y.1 } else { r_y.1 };
        if x_min > x_max || y_min > y_max {
            return false;
        }
        range_x = (x_min, x_max);
        range_y = (y_min, y_max);
    }
    true
}

fn main() {
    input! {
        n: usize,
        xyc: [(i64, i64, i64); n],
    };
    let mut ng = -1_f64;
    let mut ok = 1_000_000_000_000_f64;
    for _ in 0..100 {
        let mid = ng + (ok - ng) / 2_f64;
        if is_ok(mid, &xyc) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
