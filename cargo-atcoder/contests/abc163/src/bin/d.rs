use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let p = 1_000_000_007;
    // sum of [a, b]
    let f = |a: usize, b: usize| -> usize {
        if a > b {
            0
        } else {
            (a + b) * (b + 1 - a) / 2
        }
    };
    let mut sum = 0_usize;
    for x in k..=n + 1 {
        let min = f(0, x - 1);
        let max = f(n + 1 - x, n);
        let count = max - min + 1;
        sum += count;
        sum %= p;
    }
    let ans = sum;
    println!("{}", ans);
}
