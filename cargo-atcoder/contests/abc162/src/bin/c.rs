use proconio::input;

fn gcd(m: usize, n: usize) -> usize {
    if m < n {
        gcd(n, m)
    } else {
        let r = m % n;
        if r == 0 {
            n
        } else {
            gcd(n, r)
        }
    }
}

fn main() {
    input! {
        k: usize,
    };
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            let x = gcd(a, b);
            for c in 1..=k {
                ans += gcd(x, c);
            }
        }
    }
    println!("{}", ans);
}
