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
    let mut g_l = a.clone();
    let mut g_r = a.clone();
    for (i, a_i) in a.iter().copied().enumerate().skip(1) {
        g_l[i] = gcd(g_l[i - 1], a_i);
    }
    for (i, a_i) in a.iter().copied().enumerate().rev().skip(1) {
        g_r[i] = gcd(g_r[i + 1], a_i);
    }
    let mut ans = g_r[1].max(g_l[g_l.len() - 2]);
    for i in 1..n - 1 {
        ans = ans.max(gcd(g_l[i - 1], g_r[i + 1]));
    }
    println!("{}", ans);
}
