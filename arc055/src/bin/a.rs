use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = "0".repeat(n - 1);
    println!("1{}7", ans);
}
