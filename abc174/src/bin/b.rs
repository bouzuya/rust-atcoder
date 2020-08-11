use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };
    let ans = xy
        .iter()
        .filter(|&&(x_i, y_i)| x_i * x_i + y_i * y_i <= d * d)
        .count();
    println!("{}", ans);
}
