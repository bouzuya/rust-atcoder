use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = std::cmp::max(x, y);
    println!("{}", ans);
}
