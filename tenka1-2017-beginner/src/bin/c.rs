use proconio::input;

fn main() {
    input! {
        large_n: usize,
    };
    for h in 1..=3500 {
        for n in 1..=3500 {
            if 4 * h * n <= (h + n) * large_n {
                continue;
            }
            let w = h * n * large_n / (4 * h * n - (h + n) * large_n);
            if 4 * h * n * w != n * w * large_n + h * w * large_n + h * n * large_n {
                continue;
            }
            if w > 3500 {
                continue;
            }
            println!("{} {} {}", h, n, w);
            return;
        }
    }
}
