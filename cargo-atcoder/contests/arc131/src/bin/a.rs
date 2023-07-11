use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let x = b * 500_000_000 + a;
    let ans = x;
    println!("{}", ans);
}
