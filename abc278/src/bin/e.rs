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
    let mut map = BTreeMap::new();
    for i in 0..capital_h {
        for j in 0..capital_w {
            if (0..h).contains(&i) && (0..w).contains(&j) {
                continue;
            }
            *map.entry(a[i][j]).or_insert(0) += 1;
        }
    }

    let mut ans = vec![vec![0; (capital_w - w) + 1]; (capital_h - h) + 1];
    for i in 0..=(capital_h - h) {
        let zero = map.clone();
        ans[i][0] = map.len();

        for j in 0..=(capital_w - w) {
            if j == 0 {
                continue;
            }
            for k in 0..h {
                let key = a[i + k][j + w - 1];
                // println!("  {} {} {} {} {}", i, j, k, w, key);
                let x = map.get_mut(&key).unwrap();
                *x -= 1;
                if *x == 0 {
                    map.remove(&key);
                }

                // println!("  {} {} {} {} {} add", i, j, k, w, a[i + k][j]);
                *map.entry(a[i + k][j - 1]).or_insert(0) += 1;
            }

            ans[i][j] = map.len();
        }

        map = zero;
        // println!("{:?}", map);

        if i == (capital_h - h) {
            continue;
        }
        for j in 0..w {
            let key = a[i + h][j];
            // println!("{} {} {} {}", i, j, h, key);
            let x = map.get_mut(&key).unwrap();
            *x -= 1;
            if *x == 0 {
                map.remove(&key);
            }

            *map.entry(a[i][j]).or_insert(0) += 1;
        }
        // println!("{:?}", map);
    }

    for i in 0..=(capital_h - h) {
        for j in 0..=(capital_w - w) {
            print!(
                "{}{}",
                ans[i][j],
                if j == (capital_w - w) { '\n' } else { ' ' }
            );
        }
    }
}
