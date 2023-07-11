use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = (a * b).max(c * d);
    println!("{}", ans);
}
