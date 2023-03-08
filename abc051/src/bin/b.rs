use proconio::input;

fn main() {
    input! {
        k: usize,
        s: usize,
    };
    let mut count = 0_usize;
    for x in 0..=k.min(s) {
        for y in 0..=k.min(s) {
            let z = s.saturating_sub(x).saturating_sub(y);
            if x + y + z == s && z <= k {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
