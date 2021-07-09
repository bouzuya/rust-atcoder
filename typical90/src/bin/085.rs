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
        k: usize,
    };
    let mut count = 0_usize;
    let ds = divisors(k);
    for a in ds.iter().copied() {
        for b in ds.iter().copied() {
            let c = k / a / b;
            if a * b * c == k && a <= b && b <= c {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
