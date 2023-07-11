use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut scores = vec![0_usize; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        for d in -1..=1 {
            scores[((n + p_i - i) as i64 + d) as usize % n] += 1;
        }
    }
    let ans = scores.into_iter().max().unwrap();
    println!("{}", ans);
}
