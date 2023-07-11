use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut wp: [(i64, i64); n],
    };

    let mut ok = 0_f64;
    let mut ng = 101_f64;
    for _ in 0..100 {
        let m = ok + (ng - ok) / 2_f64;
        wp.sort_by(|&(w_a, p_a), &(w_b, p_b)| {
            let a = w_a as f64 * (p_a as f64 - m);
            let b = w_b as f64 * (p_b as f64 - m);
            b.partial_cmp(&a).unwrap()
        });
        let mut s = 0_f64;
        let mut w = 0_f64;
        for &(w_i, p_i) in wp.iter().take(k) {
            s += w_i as f64 * p_i as f64;
            w += w_i as f64;
        }
        if s / w >= m {
            ok = m;
        } else {
            ng = m;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
