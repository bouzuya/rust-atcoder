use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n - 1],
    };
    let inf = 100_000_usize;
    let mut a = vec![inf; n];
    for (i, b_i) in b.iter().copied().enumerate() {
        a[i] = a[i].min(b_i);
        a[i + 1] = a[i + 1].min(b_i);
    }
    let ans = a.into_iter().sum::<usize>();
    println!("{}", ans);
}
