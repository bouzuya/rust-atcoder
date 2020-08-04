use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut max_d = 0;
    for &(x_i, y_i) in xy.iter() {
        for &(x_j, y_j) in xy.iter() {
            let d_x = x_j - x_i;
            let d_y = y_j - y_i;
            let d = d_x * d_x + d_y * d_y;
            max_d = std::cmp::max(max_d, d);
        }
    }
    let ans = (max_d as f64).sqrt();
    println!("{}", ans);
}
