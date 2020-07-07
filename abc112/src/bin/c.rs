use proconio::input;

fn main() {
    input! {
        n: usize,
        xyh: [(i64, i64, i64); n],
    };
    for c_y in 0..=100 {
        for c_x in 0..=100 {
            let mut c_h = 0;
            for &(x_i, y_i, h_i) in xyh.iter() {
                if h_i == 0 {
                    continue;
                }
                c_h = h_i + (x_i - c_x).abs() + (y_i - c_y).abs();
                break;
            }

            let mut ok = true;
            for &(x_i, y_i, h_i) in xyh.iter() {
                if h_i != std::cmp::max(c_h - ((x_i - c_x).abs() + (y_i - c_y).abs()), 0) {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("{} {} {}", c_x, c_y, c_h);
                return;
            }
        }
    }
}
