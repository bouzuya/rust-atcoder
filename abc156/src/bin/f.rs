use proconio::input;

fn main() {
    input! {
        k: usize,
        q: usize,
        dv: [usize; k],
        qv: [(usize, usize, usize); q]
    };

    for iq in 0..q {
        let (n, x, m) = qv[iq];
        let mut eq = 0;
        let mut xj = x;
        for ik in 0..k {
            let dm = dv[ik] % m;
            let c = ((n - 1 - ik) + (k - 1)) / k;
            if dm == 0 {
                eq += c;
            } else {
                xj += dm * c;
            }
        }
        let gt = xj / m - x / m;
        let ans = (n - 1) - gt - eq;
        println!("{}", ans);
    }
}
