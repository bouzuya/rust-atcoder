use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let ans = k * (k - 1).pow((n - 1) as u32);
    println!("{}", ans);
}
