use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };
    let mut ans = 0;
    for &(x_i, y_i) in xy.iter() {
        if x_i * x_i + y_i * y_i <= d * d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
