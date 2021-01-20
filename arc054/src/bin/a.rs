use proconio::input;

fn main() {
    input! {
        l: i64,
        x: i64,
        y: i64,
        s: i64,
        d: i64,
    };
    let max = l as f64 / x as f64;
    let ans = if s <= d {
        let cw = (d - s) as f64 / (x + y) as f64;
        let ccw = if x >= y {
            max
        } else {
            (s + l - d) as f64 / (y - x) as f64
        };
        cw.min(ccw)
    } else {
        let cw = (l - s + d) as f64 / (x + y) as f64;
        let ccw = if x >= y {
            max
        } else {
            (s - d) as f64 / (y - x) as f64
        };
        cw.min(ccw)
    };
    println!("{:0.10}", ans);
}
