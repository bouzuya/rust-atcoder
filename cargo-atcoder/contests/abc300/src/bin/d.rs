use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut p = vec![];
    let mut b = vec![true; n + 1];
    for i in 2.. {
        if i * i > n {
            for j in i..=n {
                if b[j] {
                    p.push(j);
                }
            }
            break;
        }
        if b[i] {
            p.push(i);
            for j in (i + i..=n).step_by(i) {
                b[j] = false;
            }
        }
    }
    p
}

fn main() {
    input! {
        n: usize,
    };

    let f = |a: usize, b: usize, c: usize| -> Option<usize> {
        let a2 = a.checked_mul(a)?;
        let c2 = c.checked_mul(c)?;
        let x = a2.checked_mul(b)?.checked_mul(c2)?;
        if x <= n {
            Some(x)
        } else {
            None
        }
    };

    let m = n.min(1_000_000_usize);
    let ps = sieve_of_eratosthenes(m);
    let mut ans = 0_usize;
    for i_b in 1..ps.len() {
        let min_a = ps[0];
        let b = ps[i_b];
        if i_b + 1 >= ps.len() {
            continue;
        }
        let min_c = ps[i_b + 1];
        if f(min_a, b, min_c).is_none() {
            continue;
        }

        for i_c in i_b + 1..ps.len() {
            let c = ps[i_c];
            if f(min_a, b, c).is_none() {
                break;
            }
            let mut ok = 0;
            let mut ng = i_b;
            while ng - ok > 1 {
                let i_a = ok + (ng - ok) / 2;
                if f(ps[i_a], b, c).is_none() {
                    ng = i_a;
                } else {
                    ok = i_a;
                }
            }
            ans += ok + 1;
        }
    }
    println!("{}", ans);
}
