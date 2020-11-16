use proconio::input;

fn main() {
    input! {
        q: i64,
        h: i64,
        s: i64,
        d: i64,
        n: i64,
    };
    let mut ans = n * 4 * q;
    ans = std::cmp::min(ans, n * 2 * h);
    ans = std::cmp::min(ans, n * s);
    ans = std::cmp::min(
        ans,
        n / 2 * d
            + if n % 2 == 0 {
                0
            } else {
                std::cmp::min(4 * q, std::cmp::min(2 * h, s))
            },
    );
    println!("{}", ans);
}
