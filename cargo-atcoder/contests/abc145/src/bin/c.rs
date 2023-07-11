use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        xyv: [(i64, i64); n],
    };
    let mut count = 0_f64;
    let mut sum = 0_f64;
    let mut ids: Vec<usize> = (0..n).collect();
    loop {
        for i_id in 1..n {
            let i = ids[i_id - 1];
            let j = ids[i_id];
            sum += (((xyv[i].0 - xyv[j].0).pow(2) + (xyv[i].1 - xyv[j].1).pow(2)) as f64).sqrt()
        }
        count += 1_f64;
        if !ids.next_permutation() {
            break;
        }
    }
    let ans = sum / count;
    println!("{}", ans);
}
