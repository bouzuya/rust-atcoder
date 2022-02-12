use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let dist = |p1: (i64, i64), p2: (i64, i64)| -> f64 {
        (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as f64).sqrt()
    };

    let mut min = dist(xy[0], xy[1]);
    for i in 0..n {
        for j in i + 1..n {
            min = min.min(dist(xy[i], xy[j]));
        }
    }
    let ans = min;
    println!("{}", ans);
}
