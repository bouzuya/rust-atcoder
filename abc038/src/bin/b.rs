use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        h2: usize,
        w2: usize,
    };
    let ans = h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2;
    println!("{}", if ans { "YES" } else { "NO" });
}
