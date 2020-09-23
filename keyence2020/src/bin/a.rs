use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };
    let u = std::cmp::max(h, w);
    let ans = (n + u - 1) / u;
    println!("{}", ans);
}
