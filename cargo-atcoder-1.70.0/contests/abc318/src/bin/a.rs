use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    };
    let mut count = 0_usize;
    for i in 1..=n {
        if i < m {
            continue;
        }
        let d = i - m;
        if d % p == 0 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
