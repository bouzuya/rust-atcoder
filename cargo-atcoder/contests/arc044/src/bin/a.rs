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
    let ps = prime_factorization(n);
    let ans = n != 1 && ((ps.len() == 1 && ps[0].1 == 1) || ((n % 2 != 0) && (n % 5 != 0) && (n % 3 != 0)));
    println!("{}", if ans { "Prime" } else { "Not Prime" });
}
