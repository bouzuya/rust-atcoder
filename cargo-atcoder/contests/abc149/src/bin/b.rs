use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
    };
    let na = std::cmp::max(a - k, 0);
    let nb = std::cmp::max(b - std::cmp::max(k - a, 0), 0);
    println!("{} {}", na, nb);
}
