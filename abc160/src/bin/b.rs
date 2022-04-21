use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = ((x / 500) * 1_000) + ((x % 500) / 5) * 5;
    println!("{}", ans);
}
