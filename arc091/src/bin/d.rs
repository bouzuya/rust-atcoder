use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut count = 0;
    for b in 1..=n {
        let p = n / b;
        let r = n % b;
        count += p * b.saturating_sub(k) + (r + 1).saturating_sub(k);
    }
    if k == 0 {
        count -= n;
    }
    let ans = count;
    println!("{}", ans);
}
