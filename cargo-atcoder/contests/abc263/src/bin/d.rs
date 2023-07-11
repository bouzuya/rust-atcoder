use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    };
    let mut left = vec![0; n];
    left[0] = a[0].min(l);
    for t in 1..n {
        left[t] = (l * (t + 1) as i64).min(left[t - 1] + a[t])
    }
    let mut right = vec![0; n];
    right[0] = a[n - 1].min(r);
    for t in 1..n {
        right[t] = (r * (t + 1) as i64).min(right[t - 1] + a[n - 1 - t])
    }
    right.reverse();

    let mut ans = left[n - 1].min(right[0]);
    for i in 0..n - 1 {
        ans = ans.min(left[i] + right[i + 1]);
    }

    println!("{}", ans);
}
