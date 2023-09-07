use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        a: i64,
        h: [i64; n],
    };

    let mut iv = h
        .iter()
        .copied()
        .enumerate()
        .map(|(i, h_i)| (i, (1000 * a + h_i * 6 - 1000 * t).abs()))
        .collect::<Vec<_>>();
    iv.sort_by_key(|&(_, v)| v);
    let ans = iv[0].0 + 1;
    println!("{}", ans);
}
