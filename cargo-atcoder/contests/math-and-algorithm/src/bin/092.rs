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
    };
    let ds = divisors(n);
    let ans = ds.into_iter().map(|d| (d + n / d) * 2).min().unwrap();
    println!("{}", ans);
}
