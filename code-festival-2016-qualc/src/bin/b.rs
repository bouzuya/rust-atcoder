use proconio::input;

fn main() {
    input! {
        _: usize,
        t: usize,
        mut a: [i64; t]
    };
    a.sort();
    let mut l = 0;
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
    let ans = std::cmp::max(0, a[l] - 1);
    println!("{}", ans);
}
