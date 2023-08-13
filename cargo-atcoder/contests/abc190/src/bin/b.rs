use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n],
    };
    let ans = xy.iter().any(|&(x, y)| x < s && y > d);
    println!("{}", if ans { "Yes" } else { "No" });
}
