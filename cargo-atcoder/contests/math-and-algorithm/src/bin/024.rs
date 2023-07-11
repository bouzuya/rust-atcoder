use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(f64, f64); n],
    };
    let e = pq.into_iter().map(|(p, q)| q / p).sum::<f64>();
    let ans = e;
    println!("{}", ans);
}
