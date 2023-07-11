use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = n == m;
    println!("{}", if ans { "Yes" } else { "No" });
}
