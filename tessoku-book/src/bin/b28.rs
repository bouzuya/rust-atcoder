use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let p = 1_000_000_007_usize;
    let mut x = (1_usize, 1_usize);
    for _ in 0..n - 2 {
        let (a, b) = x;
        x = (b % p, (a + b) % p);
    }

    let ans = x.1;
    println!("{}", ans);
}
