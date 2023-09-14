use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xc: [(i64, i64); n],
    };
    xc.sort_by_key(|&(x, c)| (c, x));

    let mut map = BTreeMap::new();
    for (x, c) in xc {
        let (min, max) = map.entry(c).or_insert((x, x));
        *min = (*min).min(x);
        *max = (*max).max(x);
    }

    let mut ans = 0_i64;
    let mut prev = ((0_i64, 0_i64), (0_i64, 0_i64));
    for (_, (min, max)) in map {
        ans += (max - min).abs();
        let (pos, dist) = prev;
        prev = (
            (min, max),
            (
                // prev -> max -> min
                (dist.0 + (pos.0 - max).abs()).min(dist.1 + (pos.1 - max).abs()),
                // prev -> min -> max
                (dist.0 + (pos.0 - min).abs()).min(dist.1 + (pos.1 - min).abs()),
            ),
        );
    }

    let (pos, dist) = prev;
    let ans = ans + (pos.0.abs() + dist.0).min(pos.1.abs() + dist.1);
    println!("{}", ans);
}
