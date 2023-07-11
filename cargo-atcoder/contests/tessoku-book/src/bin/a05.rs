use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut count = 0_usize;
    for a in 1..=n {
        for b in 1..=n {
            let c = k.saturating_sub(a + b);
            if !(1..=n).contains(&c) {
                continue;
            }
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
