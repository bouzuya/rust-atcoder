use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64
    };
    let ans = if b % a == 0 { a + b } else { b - a };
    println!("{}", ans);
}
