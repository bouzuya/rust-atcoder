use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(i64, i64, i64); n - 1],
    };
    for i in 0..n - 1 {
        let mut t = csf[i].1 + csf[i].0;
        for &(c_j, s_j, f_j) in csf.iter().skip(i + 1) {
            if t < s_j {
                t = s_j;
            } else if t >= s_j {
                if (t - s_j) % f_j == 0 {
                    // do nothing
                } else {
                    t += f_j - (t - s_j) % f_j;
                }
            } else {
                unreachable!();
            }
            t += c_j;
        }
        println!("{}", t);
    }
    println!("{}", 0);
}
