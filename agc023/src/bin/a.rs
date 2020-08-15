use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();
    let mut m = std::collections::BTreeMap::new();
    for &s_i in s.iter() {
        let entry = m.entry(s_i).or_insert(0);
        *entry += 1;
    }
    let ans = m
        .values()
        .filter(|&&v| v >= 2)
        .map(|&v| v * (v - 1) / 2)
        .sum::<i64>();
    println!("{}", ans);
}
