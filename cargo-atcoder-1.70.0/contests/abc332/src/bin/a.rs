use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize, usize); n],
    };
    let mut sum = pq.into_iter().map(|(p, q)| p * q).sum::<usize>();
    sum += if sum >= s { 0 } else { k };
    let ans = sum;
    println!("{}", ans);
}
