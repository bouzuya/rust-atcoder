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
        let mut sv = vec![0usize; k + 1];
        let mut zv = vec![0usize; k + 1];
        for ik in 0..k {
            let dm = dv[ik] % m;
            sv[ik + 1] = sv[ik] + dm;
            zv[ik + 1] = zv[ik] + if dm == 0 { 1 } else { 0 };
        }
        let f = x;
        let l = x + (n - 1) / k * sv[k] + sv[(n - 1) % k];
        let gt = l / m - f / m;
        let eq = (n - 1) / k * zv[k] + zv[(n - 1) % k];
        let ans = (n - 1) - gt - eq;
        println!("{}", ans);
    }
}
