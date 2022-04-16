use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let d = a - b;
    let ans = c.saturating_sub(d);
    println!("{}", ans);
}
