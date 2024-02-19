use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    };
    let mut b = vec![a[0]];
    for i in 0..n - 1 {
        b.push(a[i + 1] - a[i]);
    }
    b.push(l - a[n - 1]);
    let mut ok = 0_usize;
    let mut ng = 1_000_000_000_000;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        let mut count = 0_usize;
        let mut sum = 0_usize;
        for b_i in b.iter().copied() {
            sum += b_i;
            if sum >= mid {
                sum = 0;
                count += 1;
            }
        }
        if count >= k + 1 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
