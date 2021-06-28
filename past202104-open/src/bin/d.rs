use proconio::input;

fn cumsum(a: &[i64]) -> Vec<i64> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };

    let s = cumsum(&a);
    for x in 1..=n - k + 1 {
        let ans = s[x + k - 1] - s[x - 1];
        println!("{}", ans);
    }
}
