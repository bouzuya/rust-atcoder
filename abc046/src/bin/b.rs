use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut x = k;
    for _ in 1..n {
        x *= k - 1;
    }
    let ans = x;
    println!("{}", ans);
}
