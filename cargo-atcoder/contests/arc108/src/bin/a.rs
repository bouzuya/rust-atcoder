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

fn main() {
    input! {
        s: usize,
        p: usize,
    };
    let ds = divisors(p);
    for n in ds {
        let m = p / n;
        if n + m == s && n * m == p {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
