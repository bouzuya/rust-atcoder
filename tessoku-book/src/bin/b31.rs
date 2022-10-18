use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n / 3) + (n / 5) + (n / 7) - (n / (3 * 5)) - (n / (3 * 7)) - (n / (5 * 7))
        + (n / (3 * 5 * 7));
    println!("{}", ans);
}
