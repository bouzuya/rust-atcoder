use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n + 1],
        mut c: [i64; n + m + 1],
    };
    a.reverse();
    c.reverse();
    let mut sum = vec![0; n + m + 1];
    let mut b = vec![0; m + 1];
    for i in 0..m + 1 {
        b[i] = (c[i] - sum[i]) / a[0];
        for j in 0..n + 1 {
            sum[i + j] += b[i] * a[j];
        }
    }
    for i in (0..m + 1).rev() {
        println!("{}", b[i]);
    }
}
