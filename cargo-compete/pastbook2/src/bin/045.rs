use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ok = 0_f64;
    let mut ng = 1_000_000_001_f64;
    for _ in 0..100 {
        let mid = ok + (ng - ok) / 2_f64;
        let b = a
            .iter()
            .copied()
            .map(|a_i| a_i as f64 - mid)
            .collect::<Vec<f64>>();
        let mut dp = (0_f64, 0_f64);
        for b_i in b.iter().copied() {
            dp = (dp.1, dp.0.max(dp.1) + b_i);
        }
        if dp.0.max(dp.1) >= 0_f64 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);

    let mut ok = 0_i64;
    let mut ng = 1_000_000_001_i64;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2_i64;
        let b = a
            .iter()
            .copied()
            .map(|a_i| if a_i >= mid { 1 } else { -1 })
            .collect::<Vec<i64>>();
        let mut dp = (0_i64, 0_i64);
        for b_i in b.iter().copied() {
            dp = (dp.1, dp.0.max(dp.1) + b_i);
        }
        if dp.0.max(dp.1) > 0_i64 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
