use proconio::input;

fn divisors(n: usize) -> usize {
    let mut count = 0;
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let d_x = divisors(x);
    let d_y = divisors(y);
    let ans = match d_x.cmp(&d_y) {
        std::cmp::Ordering::Greater => 'X',
        std::cmp::Ordering::Less => 'Y',
        std::cmp::Ordering::Equal => 'Z',
    };
    println!("{}", ans);
}
