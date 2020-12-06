use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let p = 1_000_000_007_usize;
    let ans = ((a * b) % p * c) % p;
    println!("{}", ans);
}
