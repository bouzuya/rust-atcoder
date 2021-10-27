use proconio::input;

fn f(
    a: &[usize],
    c: &[usize],
    inf: usize,
    memo: &mut [Vec<Option<usize>>],
    l: usize,
    r: usize,
) -> usize {
    if l == r {
        return 0;
    }

    if let Some(v) = memo[l][r] {
        return v;
    }

    let mut min = inf;
    for m in l..r {
        min = min.min(f(a, c, inf, memo, l, m) + f(a, c, inf, memo, m + 1, r) + c[r + 1] - c[l]);
    }
    memo[l][r] = Some(min);
    min
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let inf = 1_000_000_000_000_000;
    let mut memo = vec![vec![None; n]; n];
    let ans = f(&a, &c, inf, &mut memo, 0, n - 1);
    println!("{}", ans);
}
