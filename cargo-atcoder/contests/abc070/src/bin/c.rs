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
        t: [usize; n],
    };

    let mut lcm = t[0];
    for t_i in t.into_iter().skip(1) {
        lcm = lcm / gcd(lcm, t_i) * t_i;
    }
    let ans = lcm;
    println!("{}", ans);
}
