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
    };
    let mut count = vec![0_usize; n + 1];
    for x in 1..=n {
        count[x] = divisors(x).len();
    }
    let mut ans = 0_usize;
    for x in 1..=n {
        let y = n - x;
        ans += count[x] * count[y];
    }
    println!("{}", ans);
}
