use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut s = 0;
    let mut m = 1_000_i64;
    for i in 0..n {
        if i == n - 1 {
            m += a[i] * s;
        }
        for j in i + 1..n {
            if a[i] == a[j] {
                continue;
            } else if a[i] < a[j] {
                if s == 0 {
                    s = m / a[i];
                    m -= a[i] * s;
                }
                break;
            } else if a[i] > a[j] {
                m += a[i] * s;
                s = 0;
                break;
            }
        }
    }
    let ans = m;
    println!("{}", ans);
}
