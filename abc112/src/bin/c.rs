use proconio::input;

fn main() {
    input! {
        n: usize,
        xyh: [(i64, i64, i64); n],
    };
    let (x_i, y_i, h_i) = xyh.iter().copied().find(|&(_, _, h_i)| h_i > 0).unwrap();
    for c_x in 0..=100 {
        for c_y in 0..=100 {
            let h = h_i + (x_i - c_x).abs() + (y_i - c_y).abs();
            let mut ok = true;
            for (x_j, y_j, h_j) in xyh.iter().copied() {
                if (h - (x_j - c_x).abs() - (y_j - c_y).abs()).max(0) != h_j {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("{} {} {}", c_x, c_y, h);
                return;
            }
        }
    }
    unreachable!();
}
