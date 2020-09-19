use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = 0_usize;
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    sum -= n;
    let ans = if n == sum {
        "Perfect"
    } else if n > sum {
        "Deficient"
    } else if n < sum {
        "Abundant"
    } else {
        unreachable!();
    };
    println!("{}", ans);
}
