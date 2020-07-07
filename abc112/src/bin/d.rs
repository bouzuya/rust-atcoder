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
    // d.sort();
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let d = divisors(m);
    let mut ans = 1;
    for &d_i in d.iter() {
        if n * d_i <= m {
            ans = std::cmp::max(ans, d_i);
        }
    }
    println!("{}", ans);
}
