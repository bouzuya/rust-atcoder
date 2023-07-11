use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (n - 2) * 180;
    println!("{}", ans);
}
