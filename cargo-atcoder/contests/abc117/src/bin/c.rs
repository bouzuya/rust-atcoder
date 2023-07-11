use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; m],
    };
    x.sort();
    let mut d = vec![0_i64; m - 1];
    for i in 0..m - 1 {
        d[i] = x[i + 1] - x[i];
    }
    d.sort();
    let mut sum = 0;
    for i in 0..(m - 1).saturating_sub(n - 1) {
        sum += d[i as usize];
    }
    let ans = sum;
    println!("{}", ans);
}
