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
        n: usize,
        m: usize,
    };

    let mut ans = 1_usize;
    for d in divisors(m) {
        let g = m / d;
        if g >= n {
            ans = ans.max(d);
        }
        if d >= n {
            ans = ans.max(g);
        }
    }
    println!("{}", ans);
}
