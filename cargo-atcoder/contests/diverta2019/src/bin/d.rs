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
    let mut ans = 0;
    for &d_i in divisors(n).iter() {
        if d_i == 1 {
            continue;
        }
        if n % (d_i - 1) == n / (d_i - 1) {
            ans += d_i - 1;
        }
    }
    println!("{}", ans);
}
