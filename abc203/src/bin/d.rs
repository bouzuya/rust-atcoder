use proconio::input;

fn is_ok(n: usize, k: usize, a: &[Vec<usize>], x: usize) -> bool {
    let mut s = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            s[i][j] += s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1]
                + if a[i - 1][j - 1] > x { 1 } else { 0 };
        }
    }
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            if i + k <= n
                && j + k <= n
                && s[i + k][j + k] + s[i][j] - s[i + k][j] - s[i][j + k] < (k * k / 2 + 1)
            {
                return true;
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    };

    let mut ng = -1_i64;
    let mut ok = 1_000_000_000_i64;
    while ok - ng > 1 {
        let x = (ng + ok) / 2;
        if is_ok(n, k, &a, x as usize) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
