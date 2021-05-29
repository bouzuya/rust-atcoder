use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: u64,
        a: [u64; n],
    };
    let f = |n: usize, a: &[u64]| -> Vec<(usize, u64)> {
        let mut b = vec![];
        for bits in 0..1 << n {
            let is = (0..n)
                .filter(|i| (bits >> i) & 1 == 1)
                .collect::<Vec<usize>>();
            let s = is.iter().map(|&i| a[i]).sum::<u64>();
            b.push((is.len(), s));
        }
        b
    };
    let a1 = f(n / 2, &a[0..n / 2]);
    let mut a2 = f(n - n / 2, &a[n / 2..]);
    a2.sort();

    let mut count = 0_usize;
    for (k1, s1) in a1 {
        if k1 > k || s1 > p {
            continue;
        }
        if k1 == k {
            count += 1;
            continue;
        }
        let l = a2.lower_bound_by_key(&(k - k1), |(k2, _)| *k2);
        let r = a2.upper_bound_by_key(&(k - k1), |(k2, _)| *k2);
        count += a2[l..r].upper_bound_by_key(&(p - s1), |(_, s2)| *s2);
    }

    let ans = count;
    println!("{}", ans);
}
