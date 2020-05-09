use proconio::input;

fn gcd(m: u64, n: u64) -> u64 {
    if n == 0 {
        m
    } else if n > m {
        gcd(n, m)
    } else {
        gcd(n, m % n)
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let ans = a.iter().fold(a[0], |acc, &a_i| gcd(acc, a_i));
    println!("{}", ans);
}
