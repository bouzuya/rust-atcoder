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

fn main() {
    input! {
        n: usize,
    };
    let mut c = prime_factorization(n)
        .iter()
        .map(|(_, c)| *c)
        .sum::<usize>();
    c -= 1;
    let mut ans = 0;
    while c > 0 {
        c /= 2;
        ans += 1;
    }
    println!("{}", ans);
}
