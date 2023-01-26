use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    if a.contains(&0) {
        println!("{}", n);
        return;
    }

    let mut ans = 0_usize;
    let mut p = 1_usize;
    let mut r = 0;
    for l in 0..n {
        while r < n && p * a[r] <= k {
            p *= a[r];
            r += 1;
        }
        ans = ans.max(r - l);
        if r == l {
            r += 1;
        } else {
            p /= a[l];
        }
    }
    println!("{}", ans);
}
