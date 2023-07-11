use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = if n <= 1_000 - 1 {
        n
    } else if n <= 10_000 - 1 {
        n / 10 * 10
    } else if n <= 100_000 - 1 {
        n / 100 * 100
    } else if n <= 1_000_000 - 1 {
        n / 1_000 * 1_000
    } else if n <= 10_000_000 - 1 {
        n / 10_000 * 10_000
    } else if n <= 100_000_000 - 1 {
        n / 100_000 * 100_000
    } else if n <= 1_000_000_000 - 1 {
        n / 1_000_000 * 1_000_000
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
