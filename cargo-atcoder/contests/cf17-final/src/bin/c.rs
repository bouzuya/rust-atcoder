use std::cmp;

use proconio::input;

fn dfs(c: &[usize], d_i: usize, p: &mut Vec<bool>, res: &mut usize) {
    if d_i > 12 {
        let x = p
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(j, _)| j)
            .collect::<Vec<usize>>();
        let mut min = 24;
        let mut prev = 0;
        for x in x.iter().copied().skip(1) {
            min = cmp::min(min, x - prev);
            prev = x;
        }
        min = cmp::min(min, 24 - prev);
        *res = cmp::max(*res, min);
        return;
    }

    if d_i == 0 || d_i == 12 {
        p[d_i] = c[d_i] == 1;
        dfs(c, d_i + 1, p, res)
    } else {
        match c[d_i] {
            0 => dfs(c, d_i + 1, p, res),
            1 => {
                p[d_i] = true;
                dfs(c, d_i + 1, p, res);
                p[d_i] = false;
                p[24 - d_i] = true;
                dfs(c, d_i + 1, p, res);
                p[24 - d_i] = false;
            }
            2 => {
                p[d_i] = true;
                p[24 - d_i] = true;
                dfs(c, d_i + 1, p, res);
                p[d_i] = false;
                p[24 - d_i] = false;
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    input! {
        n: usize,
        d: [i64; n],
    };
    let mut c = vec![0; 12 + 1];
    c[0] += 1;
    for d_i in d {
        c[d_i as usize] += 1;
    }

    if c.iter()
        .copied()
        .enumerate()
        .any(|(d_i, count)| count >= 3 || (d_i == 0 && count >= 2) || (d_i == 12 && count >= 2))
    {
        println!("0");
        return;
    }

    let mut p = vec![false; 24 + 1];
    let mut ans = 0;
    dfs(&c, 0, &mut p, &mut ans);
    println!("{}", ans);
}
