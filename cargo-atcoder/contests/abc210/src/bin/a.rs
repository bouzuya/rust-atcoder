use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    };
    let ans = if n <= a { n * x } else { a * x + (n - a) * y };
    println!("{}", ans);
}
