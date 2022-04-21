use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n],
    };
    let mut b = vec![];
    b.push(k - a[n - 1] + a[0]);
    let mut p = a[0];
    for a_i in a.iter().copied().skip(1) {
        b.push(a_i - p);
        p = a_i;
    }
    b.sort();
    let ans = b.iter().take(n - 1).sum::<usize>();
    println!("{}", ans);
}
