use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n <= 999 { "ABC" } else { "ABD" };
    println!("{}", ans);
}
