use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let ans = 100 - (x % 100);
    println!("{}", ans);
}
