use proconio::input;

fn divisors(n: usize) -> Vec<usize> {
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            d.push(i);
            if i != n / i {
                d.push(n / i);
            }
        }
    }
    d.sort();
    d
}

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };

    let mut b = vec![];
    let mut prev = a[0];
    for a_i in a.iter().copied().skip(1) {
        b.push(a_i - prev);
        prev = a_i;
    }

    let mut g = gcd(b[0], if b.len() == 1 { b[0] } else { b[1] });
    for b_i in b {
        g = gcd(g, b_i);
    }

    let mut ans = vec![];
    for d in divisors(g) {
        if (n - 1) * d >= a[k - 1] - a[0] {
            ans.push(d);
        }
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}
