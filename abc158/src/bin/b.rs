use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };
    let m = n % (a + b);
    let ans = if b == 0 {
        n
    } else {
        a * (n / (a + b)) + std::cmp::min(a, m)
    };
    println!("{}", ans);
}
