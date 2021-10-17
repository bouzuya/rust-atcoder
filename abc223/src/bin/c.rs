use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let s = ab
        .iter()
        .map(|&(a, b)| a as f64 / b as f64)
        .collect::<Vec<f64>>();
    let sum_s = s.iter().sum::<f64>();
    let mut sum = 0_f64;
    for (i, s_i) in s.iter().copied().enumerate() {
        sum += s_i;
        if sum >= sum_s / 2_f64 {
            let s_l = sum - s_i;
            let s = sum_s / 2_f64 - s_l;
            let b_l = ab[0..i].iter().map(|(a, _)| a).sum::<i64>();
            let ans = b_l as f64 + ab[i].1 as f64 * s;
            println!("{}", ans);
            break;
        }
    }
}
