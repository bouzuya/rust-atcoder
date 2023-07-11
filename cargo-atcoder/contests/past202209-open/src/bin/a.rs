use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };
    let ans = if z + x < y { y } else { z + x };
    println!("{}", ans);
}
