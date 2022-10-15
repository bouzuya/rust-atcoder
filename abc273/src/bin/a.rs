use proconio::input;

fn f(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * f(n - 1)
    }
}

fn main() {
    input! {
        n: usize,
    };
    let ans = f(n);
    println!("{}", ans);
}
