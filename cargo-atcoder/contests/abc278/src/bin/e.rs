use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        capital_h: usize,
        capital_w: usize,
        _n: usize,
        h: usize,
        w: usize,
        a: [[usize; capital_w]; capital_h],
    };

    let mut count = BTreeMap::new();
    for i in 0..capital_h {
        for j in 0..capital_w {
            *count.entry(a[i][j]).or_insert(0) += 1;
        }
    }
    for i in 0..h {
        for j in 0..w {
            if let Some(c) = count.get_mut(&a[i][j]) {
                *c -= 1;
                if *c == 0 {
                    count.remove(&a[i][j]);
                }
            }
        }
    }

    let mut ans = vec![vec![0_usize; capital_w - w + 1]; capital_h - h + 1];

    for k in 0..=capital_h - h {
        let backup = count.clone();

        for l in 0..=capital_w - w {
            ans[k][l] = count.len();
            for i in k..k + h {
                let j = l + w;
                if j < capital_w {
                    if let Some(c) = count.get_mut(&a[i][j]) {
                        *c -= 1;
                        if *c == 0 {
                            count.remove(&a[i][j]);
                        }
                    }
                }
            }
            for i in k..k + h {
                let j = l;
                *count.entry(a[i][j]).or_insert(0) += 1;
            }
        }

        count = backup;
        for j in 0..w {
            let i = k + h;
            if i < capital_h {
                if let Some(c) = count.get_mut(&a[i][j]) {
                    *c -= 1;
                    if *c == 0 {
                        count.remove(&a[i][j]);
                    }
                }
            }
        }
        for j in 0..w {
            let i = k;
            *count.entry(a[i][j]).or_insert(0) += 1;
        }
    }

    for k in 0..=capital_h - h {
        for l in 0..=capital_w - w {
            print!(
                "{}{}",
                ans[k][l],
                if l == capital_w - w { '\n' } else { ' ' }
            );
        }
    }
}
