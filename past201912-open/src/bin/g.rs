use proconio::input;
use std::cmp::{max, min};

fn dfs(v: &Vec<Vec<i64>>, g: &mut Vec<usize>) -> i64 {
    if g.len() == v.len() {
        let mut sum = 0;
        for (i, g_i) in g.iter().enumerate() {
            for (j, g_j) in g.iter().enumerate() {
                if i == j || g_i != g_j {
                    continue;
                }
                sum += v[min(i, j)][max(i, j)];
            }
        }
        return sum;
    }
    (0..3)
        .map(|i| {
            g.push(i);
            let value = dfs(v, g);
            g.pop();
            value
        })
        .max()
        .unwrap()
}

fn main() {
    input! {
        n: usize,
    };
    let mut v = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..i + 1 {
            v[i].push(0);
        }
        input! {
            mut a: [i64; n - i - 1]
        };
        v[i].append(&mut a);
    }

    let ans = dfs(&v, &mut vec![]) / 2;
    println!("{}", ans);
}
