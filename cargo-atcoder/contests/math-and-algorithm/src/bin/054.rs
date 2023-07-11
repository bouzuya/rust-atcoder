use proconio::input;

type Matrix = Vec<Vec<usize>>;

fn mul(a: Matrix, b: Matrix) -> Matrix {
    let h = a.len();
    let w = b[0].len();
    if a[0].len() != b.len() {
        unreachable!();
    }
    let len = b.len();
    let mut ab = vec![vec![0; w]; h];
    for r in 0..h {
        for c in 0..w {
            for i in 0..len {
                ab[r][c] += a[r][i] * b[i][c];
                ab[r][c] %= 1_000_000_000;
            }
        }
    }
    ab
}

fn pow(a: Matrix, n: usize) -> Matrix {
    let mut p = vec![vec![1, 0], vec![0, 1]];
    let mut q = a;
    for i in 0..60 {
        if (n & (1 << i)) != 0 {
            p = mul(p, q.clone());
        }
        q = mul(q.clone(), q);
    }
    p
}

fn resolve(n: usize) -> usize {
    let m = vec![vec![1, 1], vec![1, 0]];
    let x = pow(m, n - 1);
    (x[1][0] + x[1][1]) % 1_000_000_000
}

fn main() {
    input! {
        n: usize,
    };
    let ans = resolve(n);
    println!("{}", ans);
}
