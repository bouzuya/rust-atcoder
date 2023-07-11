// <https://drken1215.hatenablog.com/entry/2018/06/26/125054>
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut ans = 0;
    let mut r = 0;
    let mut sum = 0;
    for l in 0..n {
        while (r < n) && (sum ^ a[r] == sum + a[r]) {
            sum += a[r];
            r += 1;
        }
        ans += r - l;
        if l == r {
            r += 1;
        } else {
            sum -= a[l];
        }
    }
    println!("{}", ans);
}
