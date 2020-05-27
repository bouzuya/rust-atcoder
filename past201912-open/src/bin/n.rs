use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        c: i64,
        lrp: [(i64, i64, i64); n],
    };

    let mut xp = std::collections::BTreeMap::new();
    for &(l_i, r_i, p_i) in lrp.iter() {
        *xp.entry(std::cmp::max(0, l_i - c + 1)).or_insert(0) += p_i;
        *xp.entry(r_i).or_insert(0) -= p_i;
    }

    let mut ans = 1_000_000_000_000_000_000;
    let mut cp = 0;
    for (&x, &dp) in xp.iter() {
        if x < w - c {
            cp += dp;
            ans = std::cmp::min(ans, cp);
        }
    }
    println!("{}", ans);
}
