use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n + (1000 - 1)) / 1000 * 1000 - n;
    println!("{}", ans);
}
