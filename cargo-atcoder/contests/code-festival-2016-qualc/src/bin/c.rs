use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i64; n],
        a: [i64; n],
    };
    let mut f = vec![0_i64; n];
    f[0] = t[0];
    let mut p = t[0];
    for (i, &t_i) in t.iter().enumerate().skip(1) {
        if p < t_i {
            f[i] = t_i;
        }
        p = t_i;
    }
    if f[n - 1] > 0 && f[n - 1] != a[n - 1] {
        println!("0");
        return;
    }
    f[n - 1] = a[n - 1];
    p = a[n - 1];
    for (i, &a_i) in a.iter().enumerate().rev().skip(1) {
        if f[i] > 0 && f[i] > a_i {
            println!("0");
            return;
        }
        if p < a_i {
            if f[i] > 0 && f[i] != a_i {
                println!("0");
                return;
            }
            f[i] = a_i;
        }
        p = a_i;
    }

    let mut ans = 1_i64;
    for (&f_i, (&t_i, &a_i)) in f.iter().zip(t.iter().zip(a.iter())) {
        if f_i == 0 {
            ans *= std::cmp::min(t_i, a_i);
            ans %= 1_000_000_007;
        }
    }
    println!("{}", ans);
}
