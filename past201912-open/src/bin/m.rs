use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    };

    let mut mp = ab
        .iter()
        .map(|&(a, b)| (a, b, false))
        .chain(cd.iter().map(|&(c, d)| (c, d, true)))
        .collect::<Vec<_>>();

    let mut ok = 0_f64;
    let mut ng = 1_000_000_000_000_f64;
    while ng - ok > 0.0000001_f64 {
        let x = ok + (ng - ok) / 2_f64;
        mp.sort_by(|&(m1, p1, _), &(m2, p2, _)| {
            (p2 as f64 - m2 as f64 * x)
                .partial_cmp(&(p1 as f64 - m1 as f64 * x))
                .unwrap()
        });
        let mut used_cd = false;
        let mut mp5 = vec![];
        for &(m, p, is_cd) in mp.iter() {
            if mp5.len() == 5 {
                break;
            }
            if used_cd && is_cd {
                continue;
            }
            mp5.push((m, p));
            used_cd |= is_cd;
        }
        let (m, p) = mp5
            .iter()
            .fold((0, 0), |acc, &(m, p)| (acc.0 + m, acc.1 + p));
        if p as f64 / m as f64 >= x as f64 {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
