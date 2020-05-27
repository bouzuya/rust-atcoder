use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        c: i64,
        lrp: [(i64, i64, i64); n],
    };

    let mut xp = vec![];
    for &(l_i, r_i, p_i) in lrp.iter() {
        xp.push((l_i - c, p_i));
        xp.push((r_i, -p_i));
    }
    xp.push((0, 0));
    xp.push((w - c, 0));
    xp.sort();

    let mut ans = std::i64::MAX;
    let mut cp = 0;
    for &(x, dp) in xp.iter() {
        cp += dp;
        if (0..=w - c).contains(&x) {
            ans = std::cmp::min(ans, cp);
        }
    }
    println!("{}", ans);
}
