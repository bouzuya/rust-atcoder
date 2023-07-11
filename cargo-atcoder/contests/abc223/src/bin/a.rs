use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = x > 0 && x % 100 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
