use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
    };
    let mut count = 0_usize;
    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                if a + b + c <= s && a * b * c <= t {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
