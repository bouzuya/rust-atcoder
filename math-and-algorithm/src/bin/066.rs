use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64
    };
    let mut count = 0_i64;
    let k1 = k - 1;
    for a in 1..=n {
        for b in 1.max(a - k1)..=(a + k - 1).min(n) {
            for c in 1.max(b - k1)..=(b + k - 1).min(n) {
                if (a - c).abs() <= k1 {
                    count += 1;
                }
            }
        }
    }
    let ans = n.pow(3) - count;
    println!("{}", ans);
}
