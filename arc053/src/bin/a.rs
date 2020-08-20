use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let ans = (h - 1) * w + h * (w - 1);
    println!("{}", ans);
}
