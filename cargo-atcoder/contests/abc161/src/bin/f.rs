use proconio::input;

fn divisors(n: i64) -> Vec<i64> {
    let mut dv = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            dv.push(i);
            if n / i != i {
                dv.push(n / i);
            }
        }
    }
    dv
}

fn main() {
    input! { n: i64 };
    let mut ans = 0;
    for &k in divisors(n).iter() {
        if k == 1 {
            continue;
        }
        let mut m = n;
        while m % k == 0 {
            m /= k;
        }
        if m % k == 1 {
            ans += 1;
        }
    }
    ans += divisors(n - 1).len() - 1;
    println!("{}", ans);
}
