use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };
    let ans = xy
        .into_iter()
        .filter(|(x, y)| x.pow(2) + y.pow(2) <= d.pow(2))
        .count();
    println!("{}", ans);
}
