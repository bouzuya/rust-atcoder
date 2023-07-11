use proconio::input;

fn f(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        x * f(x - 1)
    }
}

fn main() {
    input! {
        n: usize,
    };
    let ans = f(n);
    println!("{}", ans);
}
