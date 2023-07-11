use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    for i in 0..k {
        sum += a[i];
    }
    let mut ans = sum;
    for i in k..n {
        sum += a[i];
        sum -= a[i - k];
        ans += sum;
    }

    println!("{}", ans);
}
