use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let ans = x >= 100 && x % 100 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
