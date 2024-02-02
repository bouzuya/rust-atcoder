use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    };
    let mut count = vec![0_i64; n + 1];
    for (x_i, x_j) in x.iter().copied().zip(x.iter().copied().skip(1)) {
        let (x_i, x_j) = if x_i < x_j { (x_i, x_j) } else { (x_j, x_i) };
        let len_r = (x_j - x_i) as i64;
        let len_l = n as i64 - len_r;
        count[0] += len_r;
        count[x_i] -= len_r;
        count[x_i] += len_l;
        count[x_j] -= len_l;
        count[x_j] += len_r;
        count[n] -= len_r;
    }
    for i in 0..n {
        count[i + 1] += count[i];
    }
    let ans = count.into_iter().take(n).min().unwrap();
    println!("{}", ans);
}
