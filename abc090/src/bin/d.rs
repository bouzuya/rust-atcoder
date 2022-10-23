use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut count = 0_usize;
    for b in k + 1..=n {
        let cycle_q = n / b;
        let cycle_r = n % b;
        let count_per_cycle = b.saturating_sub(k);
        count += count_per_cycle * cycle_q + cycle_r.saturating_sub(k.saturating_sub(1));
    }
    let ans = count;
    println!("{}", ans);
}
