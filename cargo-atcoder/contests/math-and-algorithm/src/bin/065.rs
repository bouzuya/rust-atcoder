use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let ans = if h == 1 || w == 1 { 1 } else { (h * w + 1) / 2 };
    println!("{}", ans);
}
