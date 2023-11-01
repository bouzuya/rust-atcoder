use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let mut count = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let a = (y_j - y_i) as f64 / (x_j - x_i) as f64;
            if (-1_f64..=1_f64).contains(&a) {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
