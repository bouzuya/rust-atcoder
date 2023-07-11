use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = if k == 1 { 0 } else { n - k };
    println!("{}", ans);
}
