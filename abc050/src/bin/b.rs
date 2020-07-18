use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        t: [i64; n],
        m: usize,
        px: [(Usize1, i64); m],
    };
    let sum = t.iter().sum::<i64>();
    for &(p_i, x_i) in px.iter() {
        let ans = sum - t[p_i] + x_i;
        println!("{}", ans);
    }
}
