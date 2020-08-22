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
    let mut c = 0;
    for (i, (&a_i, &s_i)) in a.iter().zip(s.iter()).enumerate() {
        if s_i * 2 < a_i {
            c = i;
        }
    }
    let ans = n - c;
    println!("{}", ans);
}
