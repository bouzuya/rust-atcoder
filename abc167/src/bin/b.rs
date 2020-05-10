use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    };
    let x_a = std::cmp::min(k, a);
    let x_b = std::cmp::min(k - x_a, b);
    let x_c = std::cmp::min(k - x_a - x_b, c);
    let ans = 1 * x_a + 0 * x_b + -1 * x_c;
    println!("{}", ans);
}
