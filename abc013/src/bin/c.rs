use proconio::input;

fn main() {
    input! {
        n: i64,
        h: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
    };
    let mut ans = 1_000_000_000_000_000_000_i64;
    for x in 0..=n {
        let mut y = std::cmp::max(0, ((n - x) * e - b * x - h + (d + e - 1)) / (d + e));
        if h + b * x + d * y - e * (n - (x + y)) == 0 {
            y += 1;
        }
        if x + y > n || h + b * x + d * y - e * (n - (x + y)) <= 0 {
            continue;
        }
        ans = std::cmp::min(ans, a * x + c * y);
    }
    println!("{}", ans);
}
