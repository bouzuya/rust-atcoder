use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    };
    if s.iter().any(|&s_i| s_i == 0) {
        println!("{}", n);
        return;
    }

    let mut product = 1_usize;
    let mut max = 0_usize;
    let mut r = 0;
    for l in 0..n {
        while (r < n) && (product.checked_mul(s[r]).is_some() && product * s[r] <= k) {
            product *= s[r];
            r += 1;
        }
        max = max.max(r - l);
        if r == l {
            r += 1;
        } else {
            product /= s[l];
        }
    }
    let ans = max;
    println!("{}", ans);
}
