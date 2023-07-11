use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut sum1 = 0_i64;
    let mut sum = 0_i64;
    for i in 0..m - 1 {
        sum += a[i] * (i as i64 + 1);
        sum1 += a[i];
    }

    let mut ans = -(1 << 60);
    let mut first = true;
    for r in m - 1..n {
        if first {
            first = false;
            sum += a[r] * (m as i64);
            sum1 += a[r];
        } else {
            sum += a[r] * (m as i64);
            sum -= sum1;
            sum1 += a[r];
            sum1 -= a[r - m];
        }

        ans = ans.max(sum);
    }

    println!("{}", ans);
}
