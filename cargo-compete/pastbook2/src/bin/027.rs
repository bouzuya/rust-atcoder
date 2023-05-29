use proconio::input;

fn is_ok(ab: &mut [(i64, i64)], cd: &mut [(i64, i64)], x: f64) -> bool {
    ab.sort_by(|&(a1, b1), &(a2, b2)| {
        (b1 as f64 - x * a1 as f64)
            .partial_cmp(&(b2 as f64 - x * a2 as f64))
            .unwrap()
    });
    cd.sort_by(|&(c1, d1), &(c2, d2)| {
        (d1 as f64 - x * c1 as f64)
            .partial_cmp(&(d2 as f64 - x * c2 as f64))
            .unwrap()
    });
    ab.iter()
        .copied()
        .rev()
        .take(5)
        .map(|(a, b)| (b as f64 - x * a as f64))
        .sum::<f64>()
        >= 0_f64
        || ab
            .iter()
            .copied()
            .rev()
            .take(4)
            .map(|(a, b)| (b as f64 - x * a as f64))
            .sum::<f64>()
            + cd.iter()
                .copied()
                .rev()
                .take(1)
                .map(|(c, d)| (d as f64 - x * c as f64))
                .sum::<f64>()
            >= 0_f64
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(i64, i64); n],
        mut cd: [(i64, i64); m],
    }
    let mut ok = 0_f64;
    let mut ng = 1_000_000_000_f64;
    for _ in 0..100 {
        let x = ok + (ng - ok) / 2_f64;
        if is_ok(&mut ab, &mut cd, x) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
