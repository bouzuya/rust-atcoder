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
                ab[r][c] %= 1_000_000_007;
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
    let x = pow(m, n);
    (x[1][0] + x[1][1]) % 1_000_000_007
}

fn main() {
    input! {
        k: usize,
        n: usize,
    };
    let ans = match k {
        2 => resolve(n),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        _ => unreachable!(),
    };
    println!("{}", ans);
}
