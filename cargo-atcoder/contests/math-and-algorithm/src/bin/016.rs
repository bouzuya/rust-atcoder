use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut g = a[0];
    for a_i in a.into_iter().skip(1) {
        g = gcd(g, a_i);
    }
    let ans = g;
    println!("{}", ans);
}
