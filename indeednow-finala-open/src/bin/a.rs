use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();

    let mut sums = vec![];
    for i in 0..n / 2 {
        sums.push(a[i] + a[n - 1 - i]);
    }
    sums.sort();

    let ans = (sums[0] - sums[n / 2 - 1]).abs();
    println!("{}", ans);
}
