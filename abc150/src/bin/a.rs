use proconio::input;

fn main() {
    input! {
        k: usize,
        x: usize,
    };
    let ans = 500 * k >= x;
    println!("{}", if ans { "Yes" } else { "No" });
}
