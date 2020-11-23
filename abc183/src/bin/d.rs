use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    };
    let max_t = 2 * 100_000;
    let mut sum = vec![0_i64; max_t + 1];
    for (s_i, t_i, p_i) in stp {
        sum[s_i] += p_i;
        sum[t_i] -= p_i;
    }
    for i in 0..sum.len() - 1 {
        sum[i + 1] += sum[i];
    }
    let mut all = true;
    for &sum_i in sum.iter() {
        if sum_i > w {
            all = false;
        }
    }
    let ans = all;
    println!("{}", if ans { "Yes" } else { "No" });
}
