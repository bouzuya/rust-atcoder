use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    };
    let max_t = 2 * 100_000;
    let mut tbl = vec![0_i64; max_t + 5];
    for (s_i, t_i, p_i) in stp {
        tbl[s_i] += p_i;
        tbl[t_i] -= p_i;
    }
    let mut max = 0_i64;
    for i in 0..=max_t + 1 {
        tbl[i + 1] += tbl[i];
        max = std::cmp::max(max, tbl[i]);
    }

    let ans = max <= w;
    println!("{}", if ans { "Yes" } else { "No" });
}
