use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        pv: [usize; n]
    };
    let mut sum = 0;
    for i in 0..k {
        sum += pv[i] + 1;
    }
    let mut max_sum = sum;
    for i in k..pv.len() {
        sum += pv[i] + 1;
        sum -= pv[i - k] + 1;
        max_sum = std::cmp::max(max_sum, sum);
    }
    let ans = max_sum as f64 / 2f64;
    println!("{:.12}", ans);
}
