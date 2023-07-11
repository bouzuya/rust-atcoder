use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = (a + b - c).min(d);
    println!("{}", ans);
}
