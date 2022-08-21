use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    };
    let mut min = 1_000_000_000;
    for a in 0..=100 {
        for b in 0..=100 {
            if a + b * 3 == n {
                min = min.min(a * x + b * y);
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
