use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n * n * n;
    println!("{}", ans);
}
