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
        t: usize,
    };

    let ps = sieve_of_eratosthenes(10_000_000);
    for _ in 0..t {
        input! {
            n: usize
        }

        let mut ok = false;
        for p in ps.iter().copied() {
            let p2 = p * p;
            if n % p2 != 0 {
                continue;
            }

            println!("{} {}", p, n / p2);
            ok = true;
            break;
        }
        if ok {
            continue;
        }

        for q in ps.iter().copied() {
            if n % q != 0 {
                continue;
            }

            let p2 = n / q;
            let mut l = 1;
            let mut r = p2;
            while r - l > 1 {
                let mid = l + (r - l) / 2;
                if mid.checked_mul(mid).is_none() || mid * mid >= p2 {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            let p = r;
            if p * p == p2 {
                println!("{} {}", p, n / p2);
                ok = true;
                break;
            }
        }
        if !ok {
            unreachable!();
        }
    }
}
