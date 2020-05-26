use proconio::input;

fn main() {
    input! { n: usize };
    let mut a = vec![vec![0_i64; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! { a_ij: i64 };
            a[i][j] = a_ij;
            a[j][i] = a_ij;
        }
    }

    let mut ans = -1_000_000_000_000_000;
    let n_g = 3_usize;
    for ds in 0..n_g.pow(n as u32) {
        let mut x = ds;
        let mut g = vec![0; n];
        for i in (0..n).rev() {
            g[i] = x % n_g;
            x /= n_g;
        }

        let mut sum = 0;
        for i in 0..n {
            for j in i + 1..n {
                if g[i] == g[j] {
                    sum += a[i][j];
                }
            }
        }
        ans = std::cmp::max(ans, sum);
    }

    println!("{}", ans);
}
