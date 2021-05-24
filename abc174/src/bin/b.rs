use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };
    let mut count = 0;
    for (x_i, y_i) in xy {
        if x_i * x_i + y_i * y_i <= d * d {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
