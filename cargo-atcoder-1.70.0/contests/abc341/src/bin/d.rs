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
        m: usize,
        k: usize,
    };
    let lcm = n * m / gcd(n, m);
    let mut ng = 0;
    let mut ok = 1_usize << 60;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let cn = mid / n;
        let cm = mid / m;
        let cl = mid / lcm;
        if cn + cm - 2 * cl >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
