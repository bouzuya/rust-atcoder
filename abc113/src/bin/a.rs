use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = x + y / 2;
    println!("{}", ans);
}
