use proconio::input;

fn main() {
    input! {
        h1: i64,
        w1: i64,
        h2: i64,
        w2: i64,
    };
    let ans = h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2;
    println!("{}", if ans { "YES" } else { "NO" });
}
