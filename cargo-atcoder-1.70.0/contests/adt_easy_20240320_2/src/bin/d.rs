use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n <= 10_usize.pow(3) - 1 {
        n
    } else if n <= 10_usize.pow(4) - 1 {
        n / 10 * 10
    } else if n <= 10_usize.pow(5) - 1 {
        n / 100 * 100
    } else if n <= 10_usize.pow(6) - 1 {
        n / 1000 * 1000
    } else if n <= 10_usize.pow(7) - 1 {
        n / 10000 * 10000
    } else if n <= 10_usize.pow(8) - 1 {
        n / 100000 * 100000
    } else {
        n / 1000000 * 1000000
    };
    println!("{}", ans);
}
