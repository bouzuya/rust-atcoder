use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    };
    let ans = n * n - a;
    println!("{}", ans);
}
