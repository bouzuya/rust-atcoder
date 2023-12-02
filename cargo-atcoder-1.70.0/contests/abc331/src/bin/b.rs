use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    };
    let mut min = 1_000_000_000;
    for a in 0..100 {
        for b in 0..100 {
            for c in 0..100 {
                if 6 * a + 8 * b + 12 * c >= n {
                    min = min.min(s * a + m * b + l * c);
                }
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
