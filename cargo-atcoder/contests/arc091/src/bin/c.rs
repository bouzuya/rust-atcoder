use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = if n == 1 && m == 1 {
        1
    } else if n == 1 && m > 1 {
        m - 2
    } else if n > 1 && m == 1 {
        n - 2
    } else if n > 1 && m > 1 {
        n * m - (n * 2 + (m - 2) * 2)
    } else {
        unreachable!();
    };
    println!("{}", ans);
}
