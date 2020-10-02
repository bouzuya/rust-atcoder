use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = std::cmp::max(a * b, c * d);
    println!("{}", ans);
}
