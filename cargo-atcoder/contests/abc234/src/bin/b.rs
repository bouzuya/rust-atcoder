use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut max = 0_f64;
    for i in 0..n {
        for j in i + 1..n {
            if i == j {
                continue;
            }

            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let d = (((x_i - x_j).pow(2) + (y_i - y_j).pow(2)) as f64).sqrt();
            max = max.max(d);
        }
    }
    let ans = max;
    println!("{}", ans);
}
