use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut min = n * n;
    for h in 1..=n {
        for w in 1..=n / h {
            min = min.min((h.max(w) - h.min(w)) + (n - h * w));
        }
    }
    let ans = min;
    println!("{}", ans);
}
