use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = 1_f64 / a.into_iter().map(|a_i| 1_f64 / (a_i as f64)).sum::<f64>();
    println!("{}", ans);
}
