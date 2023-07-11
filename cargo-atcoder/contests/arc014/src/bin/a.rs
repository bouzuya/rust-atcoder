use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n % 2 == 0 { "Blue" } else { "Red" };
    println!("{}", ans);
}
