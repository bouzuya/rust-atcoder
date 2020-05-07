use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = std::cmp::min(c, b / a);
    println!("{}", ans);
}
