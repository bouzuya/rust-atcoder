use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = match (n == 1, m == 1) {
        (true, true) => 1,
        (true, false) | (false, true) => n * m - 2,
        (false, false) => n * m - 2 * (n + m - 2),
    };
    println!("{}", ans);
}
