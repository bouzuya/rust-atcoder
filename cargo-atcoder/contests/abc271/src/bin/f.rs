use std::collections::HashMap;

use proconio::input;

fn f(a: &[Vec<usize>]) -> Vec<Vec<HashMap<usize, usize>>> {
    let n = a.len();
    let mut count = vec![vec![HashMap::new(); n]; n];
    *count[0][0].entry(0).or_insert(0) += 1_usize;
    for i in 0..n {
        for j in 0..n {
            if i + j >= n / 2 * 2 {
                continue;
            }
            if i + 1 < n {
                let map = count[i][j]
                    .iter()
                    .map(|(&k, &c)| (k, c))
                    .collect::<Vec<(usize, usize)>>();
                let map2 = &mut count[i + 1][j];
                for (k, c) in map {
                    *map2.entry(k ^ a[i][j]).or_insert(0) += c;
                }
            }
            if j + 1 < n {
                let map = count[i][j]
                    .iter()
                    .map(|(&k, &c)| (k, c))
                    .collect::<Vec<(usize, usize)>>();
                let map2 = &mut count[i][j + 1];
                for (k, c) in map {
                    *map2.entry(k ^ a[i][j]).or_insert(0) += c;
                }
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    let count1 = f(&a);

    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[n - 1 - i][n - 1 - j] = a[i][j];
        }
    }

    let count2_ = f(&b);

    let mut count2 = vec![vec![HashMap::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            count2[n - 1 - i][n - 1 - j] = count2_[i][j].clone();
        }
    }

    let mut ans = 0_usize;
    for i in 0..n {
        for j in 0..n {
            if i + j != n / 2 * 2 {
                continue;
            }

            let mut sum = 0_usize;
            for (&k, &v1) in count1[i][j].iter() {
                match count2[i][j].get(&(k ^ a[i][j])) {
                    None => {}
                    Some(v2) => {
                        sum += v1 * v2;
                    }
                }
            }
            ans += sum;
        }
    }

    println!("{}", ans);
}
