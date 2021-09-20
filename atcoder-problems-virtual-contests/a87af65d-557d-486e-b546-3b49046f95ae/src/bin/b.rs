use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut power = 1_usize;
    for i in 1..=n {
        power *= i;
        power %= 1_000_000_007_usize;
    }
    let ans = power;
    println!("{}", ans);
}
