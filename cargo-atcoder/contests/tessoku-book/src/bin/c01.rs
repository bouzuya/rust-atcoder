use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n + n / 10;
    println!("{}", ans);
}
