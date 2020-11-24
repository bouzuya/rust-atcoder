use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        t: i64,
        a: [i64; n],
    };
    let mut b = vec![];
    for bits in 0..1 << n / 2 {
        let mut sum = 0;
        for i in 0..n / 2 {
            if (bits >> i) & 1 == 1 {
                sum += a[i];
            }
        }
        if sum <= t {
            b.push(sum);
        }
    }
    b.sort();

    let mut max_x = 0;
    let o = n / 2;
    for bits in 0..1 << n / 2 {
        let mut sum = 0;
        for i in 0..n / 2 {
            if (bits >> i) & 1 == 1 {
                sum += a[o + i];
            }
        }
        if sum <= t {
            let x = sum + b[b.lower_bound(&(t - sum))];
            if x <= t {
                max_x = std::cmp::max(max_x, x);
            }
        }
    }
    let ans = max_x;
    println!("{}", ans);
}
