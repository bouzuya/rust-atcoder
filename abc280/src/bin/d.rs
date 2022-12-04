use proconio::input;

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut p = vec![];
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.push((i, c));
        }
    }
    if n != 1 {
        p.push((n, 1));
    }
    p
}

fn f(x: usize, p: usize) -> usize {
    if x == 0 {
        return 0;
    }
    x / p + f(x / p, p)
}

fn main() {
    input! {
        k: usize,
    };

    let ps = prime_factorization(k);

    let mut ok = k;
    let mut ng = 1;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        if ps.iter().copied().all(|(p, q)| f(x, p) >= q) {
            ok = x;
        } else {
            ng = x;
        }
    }

    let ans = ok;
    println!("{}", ans);
}
