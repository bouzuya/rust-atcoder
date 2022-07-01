use proconio::{input, marker::Usize1};

fn dfs(
    res: &mut usize,
    capital_a: &mut Vec<usize>,
    n: usize,
    m: usize,
    q: usize,
    abcd: &[(usize, usize, usize, usize)],
    i: usize,
) {
    if capital_a.len() == n {
        let mut r = 0;
        for (a, b, c, d) in abcd.iter().copied() {
            if capital_a[b] - capital_a[a] == c {
                r += d;
            }
        }
        *res = (*res).max(r);
        return;
    }

    for j in i..=m {
        capital_a.push(j);
        dfs(res, capital_a, n, m, q, abcd, j);
        capital_a.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, usize, usize); q]
    };
    let mut ans = 0;
    let mut capital_a = vec![];
    dfs(&mut ans, &mut capital_a, n, m, q, &abcd, 1);
    println!("{}", ans);
}
