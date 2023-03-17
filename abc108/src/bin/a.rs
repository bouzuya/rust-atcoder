use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let ans = (k / 2 + k % 2) * (k / 2);
    println!("{}", ans);
}
