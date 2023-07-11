use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let count = a
        .into_iter()
        .zip(c.into_iter())
        .rposition(|(a_i, c_i)| a_i > c_i * 2)
        .unwrap_or(0);
    let ans = n - count;
    println!("{}", ans);
}
