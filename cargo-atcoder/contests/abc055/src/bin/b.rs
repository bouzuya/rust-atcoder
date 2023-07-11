use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut x = 1_usize;
    for i in 1..=n {
        x *= i;
        x %= 1_000_000_007_usize;
    }
    let ans = x;
    println!("{}", ans);
}
