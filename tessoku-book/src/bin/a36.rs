use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let d = (n - 1) * 2;
    let ans = k >= d && (d % 2) == (k % 2);
    println!("{}", if ans { "Yes" } else { "No" });
}
