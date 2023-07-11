use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = if k == 1 { true } else { n >= 2 * k };
    println!("{}", if ans { "YES" } else { "NO" });
}
