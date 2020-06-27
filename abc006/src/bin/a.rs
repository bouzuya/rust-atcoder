use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n % 3 == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
