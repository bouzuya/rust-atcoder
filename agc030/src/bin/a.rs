use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = b + c.min(a + b + 1);
    println!("{}", ans);
}
