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
        k: usize,
        a: [usize; n],
    };
    let max_a = a.iter().copied().max().unwrap();
    let mut g = a[0];
    for a_i in a.iter().copied().skip(1) {
        g = gcd(g, a_i);
    }
    let ans = k <= max_a && k % g == 0;
    println!("{}", if ans { "POSSIBLE" } else { "IMPOSSIBLE" });
}
