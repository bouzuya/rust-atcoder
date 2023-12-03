use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    };
    let max = ((n + 6 - 1) / 6, (n + 8 - 1) / 8, (n + 12 - 1) / 12);
    let mut min = s * max.0 + m * max.1 + l * max.2;
    for a in 0..=max.0 {
        for b in 0..=max.1 {
            for c in 0..=max.2 {
                if 6 * a + 8 * b + 12 * c >= n {
                    min = min.min(s * a + m * b + l * c);
                }
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
