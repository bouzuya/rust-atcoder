use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        lr: [(Usize1, Usize1); n],
    };
    let mut c = vec![0_i64; 100_000 + 1];
    for &(l, r) in lr.iter() {
        c[l] += 1;
        c[r + 1] -= 1;
    }
    let s = c
        .iter()
        .scan(0, |acc, &c_i| {
            *acc += c_i;
            Some(*acc)
        })
        .collect::<Vec<i64>>();
    let ans = s.len() - s.iter().filter(|&&c| c == 0).count();
    println!("{}", ans);
}
