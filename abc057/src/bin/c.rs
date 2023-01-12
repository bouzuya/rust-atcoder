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
    let mut min = n.to_string().len();
    for a in ds.iter().copied() {
        let b = n / a;
        min = min.min(a.to_string().len().max(b.to_string().len()));
    }

    let ans = min;
    println!("{}", ans);
}
