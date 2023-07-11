use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n % 2 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
