use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n / 3 + n / 5 - n / (3 * 5);
    println!("{}", ans);
}
