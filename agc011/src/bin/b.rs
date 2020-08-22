use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let c = a
        .iter()
        .zip(s.iter())
        .rposition(|(&a_i, &s_i)| s_i * 2 < a_i)
        .unwrap_or(0);
    let ans = n - c;
    println!("{}", ans);
}
