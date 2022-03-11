use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n - 1) * n / 2;
    println!("{}", ans);
}
