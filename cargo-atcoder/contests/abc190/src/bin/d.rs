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
    let mut count = 0_usize;
    for x in divisors(2 * n) {
        let y = 2 * n / x;
        if x % 2 != y % 2 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
