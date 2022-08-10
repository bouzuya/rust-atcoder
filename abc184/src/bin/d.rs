use proconio::input;

fn f(memo: &mut Vec<Vec<Vec<Option<f64>>>>, a: usize, b: usize, c: usize) -> f64 {
    if a == 100 || b == 100 || c == 100 {
        return 0_f64;
    }
    if let Some(x) = memo[a][b][c] {
        return x;
    }

    let res = (a as f64 * f(memo, a + 1, b, c)
        + b as f64 * f(memo, a, b + 1, c)
        + c as f64 * f(memo, a, b, c + 1))
        / (a + b + c) as f64
        + 1_f64;
    memo[a][b][c] = Some(res);
    res
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let mut memo = vec![vec![vec![None; 100 + 1]; 100 + 1]; 100 + 1];
    let ans = f(&mut memo, a, b, c);
    println!("{}", ans);
}
