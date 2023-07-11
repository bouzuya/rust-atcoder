use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut l = 0;
    let mut r = n - 1;
    let mut sum_l = a[l];
    let mut sum_r = a[r];
    while r - l > 1 {
        if sum_l < sum_r {
            l += 1;
            sum_l += a[l];
        } else {
            r -= 1;
            sum_r += a[r];
        }
    }
    let ans = (sum_r - sum_l).abs();
    println!("{}", ans);
}
