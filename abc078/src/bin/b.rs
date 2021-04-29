use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };
    let ans = (x - z) / (y + z);
    println!("{}", ans);
}
