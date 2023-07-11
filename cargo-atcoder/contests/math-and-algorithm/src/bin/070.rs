use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };
    let mut ans = 1_i64 << 62;
    for l in xy.iter().copied().map(|(x, _)| x) {
        for r in xy.iter().copied().map(|(x, _)| x) {
            if l >= r {
                continue;
            }
            for b in xy.iter().copied().map(|(_, y)| y) {
                for t in xy.iter().copied().map(|(_, y)| y) {
                    if b >= t {
                        continue;
                    }
                    let count = xy
                        .iter()
                        .copied()
                        .filter(|(x, y)| (l..=r).contains(x) && (b..=t).contains(y))
                        .count();
                    if count >= k {
                        ans = ans.min((r - l).abs() * (t - b).abs());
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
