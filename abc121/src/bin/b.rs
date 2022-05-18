use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i64,
        b: [i64; m],
        a: [[i64; m]; n]
    }
    let ans = a
        .into_iter()
        .filter(|a_i| {
            a_i.iter()
                .copied()
                .zip(b.iter().copied())
                .map(|(a_ij, b_j)| a_ij * b_j)
                .sum::<i64>()
                + c
                > 0
        })
        .count();
    println!("{}", ans);
}
