use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut ans = false;
    for r in 0..=n {
        for c in 0..=m {
            if r * (m - c) + c * (n - r) == k {
                ans = true;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
