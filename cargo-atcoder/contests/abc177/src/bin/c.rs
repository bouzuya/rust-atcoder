use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let p = 1_000_000_007_usize;
    let mut ans = 0_usize;
    let mut sum = a.iter().copied().sum::<usize>() % p;
    for a_i in a {
        ans += a_i * ((p + sum - a_i) % p);
        ans %= p;
        sum += p - a_i;
        sum %= p;
    }
    println!("{}", ans);
}
