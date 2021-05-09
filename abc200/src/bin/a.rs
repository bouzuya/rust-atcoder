use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n + 100 - 1) / 100;
    println!("{}", ans);
}
