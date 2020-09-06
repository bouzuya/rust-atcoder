use proconio::input;

fn f(x: usize) -> usize {
    let mut res = 0;
    let mut x = x;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}

fn main() {
    input! {
        n: usize,
    };
    let ans = n % f(n) == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
