use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n % 2 != 0 { "Black" } else { "White" };
    println!("{}", ans);
}
