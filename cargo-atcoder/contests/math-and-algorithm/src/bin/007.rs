use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    let ans = (1..=n).filter(|i| i % x == 0 || i % y == 0).count();
    println!("{}", ans);
}
