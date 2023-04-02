use proconio::input;

fn main() {
    input! {
        n: u128,
        m: u128,
    };

    if n * n < m {
        println!("-1");
        return;
    }

    let mut min = n * n;
    for a in 1.. {
        let b = (m + a - 1) / a;
        if a > b {
            break;
        }
        if a <= n && b <= n {
            min = min.min(a * b);
        }
    }

    let ans = min;
    println!("{}", ans);
}
