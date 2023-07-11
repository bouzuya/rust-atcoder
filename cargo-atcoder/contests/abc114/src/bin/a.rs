use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = x == 7 || x == 5 || x == 3;
    println!("{}", if ans { "YES" } else { "NO" });
}
