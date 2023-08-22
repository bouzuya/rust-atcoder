use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n - 1],
    };
    let mut a = vec![0; n];
    a[0] = b[0];
    for i in 0..n - 2 {
        a[i + 1] = b[i].min(b[i + 1]);
    }
    a[n - 1] = b[n - 2];
    let ans = a.iter().sum::<usize>();
    println!("{}", ans);
}
