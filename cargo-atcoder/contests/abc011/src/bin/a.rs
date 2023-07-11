use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n == 12 { 1 } else { n + 1 };
    println!("{}", ans);
}
