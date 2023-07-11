use proconio::input;

fn main() {
    input! {
        large_n: i64,
    };
    for h in 1..=3500_i64 {
        for n in 1..=3500_i64 {
            let w_d = (4_i64 * h * n) - (large_n * n) - (large_n * h);
            if w_d <= 0 {
                continue;
            }
            let w = (large_n * h * n) / w_d;
            if (1..=3500_i64).contains(&w)
                && w * (4 * h * n - large_n * n - large_n * h) == large_n * h * n
            {
                println!("{} {} {}", h, n, w);
                return;
            }
        }
    }
}
