use proconio::input;

fn f(a: &[usize], c: &[usize], dp: &mut Vec<Vec<Option<usize>>>, l: usize, r: usize) -> usize {
    if let Some(x) = dp[l][r] {
        return x;
    }

    if l == r {
        let v = 0;
        dp[l][r] = Some(v);
        return v;
    }

    let sum = c[r + 1] - c[l];
    let mut res: Option<usize> = None;
    for m in l..r {
        let v = f(a, c, dp, l, m) + f(a, c, dp, m + 1, r) + sum;
        res = Some(res.map(|x| x.min(v)).unwrap_or(v));
    }
    dp[l][r] = res;
    res.unwrap()
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut dp = vec![vec![None; n]; n];
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    f(&a, &c, &mut dp, 0, n - 1);
    let ans = dp[0][n - 1].unwrap();
    println!("{}", ans);
}
