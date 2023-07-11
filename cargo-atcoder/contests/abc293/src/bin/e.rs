use proconio::input;

#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
fn mat_mul(a: &[Vec<usize>], b: &[Vec<usize>], modulus: usize) -> Vec<Vec<usize>> {
    let a_r = a.len();
    let a_c = a[0].len();
    let b_r = b.len();
    let b_c = b[0].len();
    assert_eq!(a_c, b_r);

    let n = a_r;
    let m = a_c;
    let l = b_c;

    let mut res = vec![vec![0_usize; l]; n];
    for i in 0..n {
        for j in 0..l {
            for k in 0..m {
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] %= modulus;
            }
        }
    }
    res
}

#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
fn mat_pow(a: &[Vec<usize>], b: usize, modulus: usize) -> Vec<Vec<usize>> {
    let r = a.len();
    let c = a[0].len();
    assert_eq!(r, c);
    let n = r;

    let mut res = vec![vec![0_usize; n]; n];
    for i in 0..n {
        res[i][i] = 1_usize;
    }
    let mut a = a.to_vec();
    let mut b = b;
    while b != 0 {
        if (b & 1) != 0 {
            res = mat_mul(&res, &a, modulus);
        }
        a = mat_mul(&a, &a, modulus);
        b >>= 1;
    }
    res
}

fn main() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    };
    let a = vec![vec![a, 1], vec![0, 1]];
    let mat = mat_pow(&a, x, m);
    let ans = mat[0][1];
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat_mul() {
        let a = vec![vec![1, 1], vec![0, 0]];
        let b = vec![vec![1, 0], vec![1, 0]];
        assert_eq!(mat_mul(&a, &b, 10), vec![vec![2, 0], vec![0, 0]]);
        assert_eq!(mat_mul(&b, &a, 10), vec![vec![1, 1], vec![1, 1]]);
    }

    #[test]
    fn test_mat_pow() {
        let a = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(mat_pow(&a, 2, 10), vec![vec![2, 1], vec![1, 1]]);
        assert_eq!(mat_pow(&a, 3, 10), vec![vec![3, 2], vec![2, 1]]);
        assert_eq!(mat_pow(&a, 4, 10), vec![vec![5, 3], vec![3, 2]]);
    }
}
