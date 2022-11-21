use proconio::{input, marker::Chars};

fn f(h: &[usize], b: &[usize], p: usize, l: usize, r: usize) -> usize {
    (p + h[r] - (h[l - 1] * b[r - l + 1] % p)) % p
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    };

    let p = 2_147_483_647_usize;
    let b = {
        let mut b = vec![1];
        for i in 0..n {
            b.push(b[i] * 100 % p);
        }
        b
    };
    let mut h = vec![0];
    for (i, s_i) in s.iter().copied().enumerate() {
        let t_i = (s_i as u8 - b'a' + 1) as usize;
        h.push((100 * h[i] + t_i) % p);
    }
    let mut ih = vec![0];
    for (i, s_i) in s.iter().copied().rev().enumerate() {
        let t_i = (s_i as u8 - b'a' + 1) as usize;
        ih.push((100 * ih[i] + t_i) % p);
    }

    for (l, r) in lr {
        let ans = f(&h, &b, p, l, r) == f(&ih, &b, p, n - r + 1, n - l + 1);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
