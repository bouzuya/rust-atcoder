use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n - 1) / 100 + 1;
    println!("{}", ans);
}
