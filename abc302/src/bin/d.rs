use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        a: [i64; n],
        mut b: [i64; m],
    };

    b.sort();

    let mut max: Option<i64> = None;
    for a_i in a.iter().copied() {
        let l = b.lower_bound(&(a_i - d));
        let u = b.lower_bound(&(a_i + d + 1));
        for &j in &[l.saturating_sub(1), l, l + 1, u.saturating_sub(1), u, u + 1] {
            if (0..m).contains(&j) && (a_i - b[j]).abs() <= d {
                max = Some(match max {
                    Some(old) => old.max(b[j] + a_i),
                    None => b[j] + a_i,
                });
            }
        }
    }

    let ans = max.unwrap_or(-1);
    println!("{}", ans);
}
