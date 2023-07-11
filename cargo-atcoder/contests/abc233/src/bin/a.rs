use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = if x >= y {
        0
    } else {
        let z = y - x;
        (z + 10 - 1) / 10
    };
    println!("{}", ans);
}
