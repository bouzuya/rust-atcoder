use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: i64,
    };
    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            c_i: i64,
            a_i: [i64; m],
        };
        c.push(c_i);
        a.push(a_i);
    }
    let inf = 1_000_000_000_000_i64;
    let mut ans = inf;
    for bits in 0..2_usize.pow(n as u32) {
        let mut sa = vec![0_i64; m];
        let mut sc = 0;
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                sc += c[i];
                for (j, &a_ij) in a[i].iter().enumerate() {
                    sa[j] += a_ij;
                }
            }
        }

        if sa.iter().filter(|&&sa_ij| sa_ij >= x).count() == m {
            ans = std::cmp::min(ans, sc);
        }
    }
    println!("{}", if ans == inf { -1 } else { ans });
}
