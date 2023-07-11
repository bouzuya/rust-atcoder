use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    };
    let mut ans = 0_f64;
    for i in 0..n {
        for j in i + 1..n {
            let (l_i, r_i) = lr[i];
            let (l_j, r_j) = lr[j];
            let all = (r_i - l_i + 1) * (r_j - l_j + 1);
            let mut count = 0_i64;
            for v_i in l_i..=r_i {
                for v_j in l_j..=r_j {
                    if v_i > v_j {
                        count += 1;
                    }
                }
            }
            ans += count as f64 / all as f64;
        }
    }
    println!("{}", ans);
}
