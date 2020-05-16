use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    let ans = (a - 1) * (b - 1);
    println!("{}", ans);
}
