use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = (n - 1) * (m - 1);
    println!("{}", ans);
}
