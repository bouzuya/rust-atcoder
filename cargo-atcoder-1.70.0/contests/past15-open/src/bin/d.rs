use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        r: usize,
    };

    let mut ok = vec![0_i64; a.max(c) + r + r + 1];
    for t in 0..=a.max(c) + r {
        if t % d == 0 {
            let (p, q) = if t < b { (a, a + r) } else { (c, c + r) };
            if t < q {
                ok[p.max(t)] += 1;
                ok[q] -= 1;
            }
        }
    }
    for t in 0..a.max(c) + r + r {
        ok[t + 1] += ok[t];
    }

    let ans = ok.iter().skip(c).take(r).all(|&x| x > 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
