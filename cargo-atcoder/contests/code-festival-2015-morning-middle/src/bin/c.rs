use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut i_l = 0;
    let mut l = 0;
    let mut i_r = n - 1;
    let mut r = 0;
    let mut ans = 0;
    for _ in 0..n / 2 {
        let v_l = (l + a[i_l]) * 2 + 1 + a[i_l + 1];
        let v_r = (r + a[i_r]) * 2 + 1 + a[i_r - 1];
        if v_l < v_r {
            l += a[i_l] + a[i_l + 1] + 2;
            ans += v_l;
            i_l += 2;
        } else {
            r += a[i_r] + a[i_r - 1] + 2;
            ans += v_r;
            i_r -= 2;
        }
    }
    println!("{}", ans);
}
