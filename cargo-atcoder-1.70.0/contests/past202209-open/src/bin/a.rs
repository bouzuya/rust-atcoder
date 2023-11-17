use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };
    let ans = (x + z).max(y);
    println!("{}", ans);
}
