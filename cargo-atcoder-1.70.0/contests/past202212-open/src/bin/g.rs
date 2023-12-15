use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let mut max = a[0];
    let mut min = s[0];
    for s_i in s.iter().copied().skip(1) {
        max = max.max(s_i - min);
        min = min.min(s_i);
    }

    let ans = max;
    println!("{}", ans);
}
