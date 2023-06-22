use proconio::input;

fn mul(a: &[Vec<usize>], b: &[Vec<usize>], modp: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut c = vec![vec![0_usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= modp;
            }
        }
    }
    c
}

fn main() {
    input! {
        n: usize,
        mut k: usize,
        m: [[usize; n]; n],
    }

    let modp = 1_000_000_007_usize;

    let mut r = {
        let mut unit = vec![vec![0_usize; n]; n];
        for i in 0..n {
            unit[i][i] = 1_usize;
        }
        unit
    };
    let mut t = m;
    while k != 0 {
        if (k & 1) == 1 {
            r = mul(&r, &t, modp);
        }
        t = mul(&t, &t, modp);
        k >>= 1;
    }

    let mut ans = 0_usize;
    for i in 0..n {
        ans += r[i].iter().sum::<usize>();
        ans %= modp;
    }
    println!("{}", ans);
}
