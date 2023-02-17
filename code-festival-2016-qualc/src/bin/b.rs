use proconio::input;

fn main() {
    input! {
        _k: usize,
        t: usize,
        mut a: [usize; t],
    };
    a.sort();
    let mut l = 0_usize;
    let mut r = t - 1;
    while l < r {
        a[l] -= 1;
        a[r] -= 1;
        if a[l] == 0 {
            l += 1;
        }
        if a[r] == 0 {
            r -= 1;
        }
    }
    let ans = a[l].saturating_sub(1);
    println!("{}", ans);
}
