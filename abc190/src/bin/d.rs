use proconio::input;

fn divisors(n: i64) -> Vec<i64> {
    let mut d = vec![];
    for i in 1_i64.. {
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
        n: i64,
    };
    let mut count = 0;
    for x in divisors(2 * n) {
        let y = 2 * n / x;
        if (x % 2 == 0 && y % 2 != 0) || (x % 2 != 0 && y % 2 == 0) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
