use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut xor = 0_usize;
    let mut sum = 0_usize;
    let mut count = 0_usize;
    let mut r = 0_usize;
    for l in 0..n {
        while (r < n) && (xor ^ a[r] == sum + a[r]) {
            xor ^= a[r];
            sum += a[r];
            r += 1;
        }
        count += r - l;
        if r == l {
            r += 1;
        } else {
            xor ^= a[l];
            sum -= a[l];
        }
    }
    let ans = count;
    println!("{}", ans);
}
