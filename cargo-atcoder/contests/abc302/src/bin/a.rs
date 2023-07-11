use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a + b - 1) / b;
    println!("{}", ans);
}
