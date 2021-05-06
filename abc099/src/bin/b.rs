use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    let d = b - a;
    let sum = (1..=d).sum::<u64>();
    let ans = sum - b;
    println!("{}", ans);
}
