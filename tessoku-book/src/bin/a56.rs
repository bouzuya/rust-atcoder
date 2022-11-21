use proconio::{input, marker::Chars};

fn f(h: &[usize], b: &[usize], p: usize, l: usize, r: usize) -> usize {
    (p + h[r] - (h[l - 1] * b[r - l + 1] % p)) % p
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        abcd: [(usize, usize, usize, usize); q]
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

    let g = |l: usize, r: usize| -> usize { f(&h, &b, p, l, r) };
    for (a, b_i, c, d) in abcd {
        let ans = g(a, b_i) == g(c, d);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
