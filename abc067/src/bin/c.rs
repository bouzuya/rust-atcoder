use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |sum, a_i| {
            *sum += a_i;
            Some(*sum)
        }))
        .collect::<Vec<i64>>();
    let ans = (1..n)
        .map(|i| {
            let x = c[i];
            let y = c[n] - c[i];
            (x - y).abs()
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
