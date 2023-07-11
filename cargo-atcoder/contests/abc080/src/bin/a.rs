use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let ans = (a * n).min(b);
    println!("{}", ans);
}
