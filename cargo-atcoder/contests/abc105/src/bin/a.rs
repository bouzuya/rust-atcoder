use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = if n % k == 0 { 0 } else { 1 };
    println!("{}", ans);
}
