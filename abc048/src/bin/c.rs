use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut a: [i64; n],
    };
    let mut ans = 0_i64;
    let mut l = 0;
    let mut r = n - 1;
    while r - l > 0 && 1 <= r && l + 1 <= n - 1 {
        let s_l = a[l] + a[l + 1];
        let s_r = a[r - 1] + a[r];
        let d_l = if s_l <= x { 0 } else { s_l - x };
        let d_r = if s_r <= x { 0 } else { s_r - x };
        if d_l <= d_r {
            ans += d_l;
            a[l + 1] = std::cmp::max(0, a[l + 1] - d_l);
            l += 1;
        } else {
            ans += d_r;
            a[r - 1] = std::cmp::max(0, a[r - 1] - d_r);
            r -= 1;
        }
    }
    println!("{}", ans);
}
